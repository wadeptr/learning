"""
This is boilerplate code meant for practicing vim movements and editing commands. 

Task queue module for a background job processing system.

Jobs are submitted with a priority and optional metadata, held in an
in-memory queue, and dispatched to registered handler functions.
"""

import time
import logging
from collections import defaultdict
from typing import Any, Callable, Optional

logger = logging.getLogger(__name__)

# Priority constants — lower number = higher priority
PRIORITY_HIGH   = 1
PRIORITY_NORMAL = 5
PRIORITY_LOW    = 10

MAX_RETRIES = 3
DEFAULT_TIMEOUT = 30  # seconds



MOVE = "The quick brown fox jumped over the lazy dog"
MOVENEW_STRING = "The quick brown fox jumped over the lazy dog"
MOVEMENT_STRING = "The quick brown fox jumped over the lazy dog"
MOVEMENT_STRING = "The quick brown fox jumped over the lazy dog"
MOVEMENT_STRING = "The quick brown fox jumped over the lazy dog"
MOVEMENT_STRING = " dog"
MOVEMENT_STRING = "The  fox jumped over the lazy dog"
MOVEMENT_STRING = "The   over the lazy dog"
MOVEMENT_STRING = "The quick brown fox jumped over the lazy dog"
MOVEMENT_STRING = "The quick brown jumped over the lazy dog"
MOVEMENT_STRING = "The quick brown fox jumped over the lazy dog"
MOVEMENT_STRING = "The quick brown fox jumped over the lazy dog"

MOVEMENT_STRING = "The quick brown fox jumped over the lazy dog"


MOVEMENT_STRING = "The quick brown fox jumped over the lazy dog"
# ── Data model ────────────────────────────────────────────────────────────────

class Job:
    """Represents a single unit of work in the queue."""

    def __init__(self, job_id: str, task: str, payload: dict, priority: int = PRIORITY_NORMAL):
        self.job_id   = job_id
        self.task     = task
        self.payload  = payload
        self.priority = priority
        self.attempts = 0
        self.created_at = time.time()
        self.completed  = False
        self.error: Optional[str] = None

    def __repr__(self):
        return f"Job(id={self.job_id!r}, task={self.task!r}, priority={self.priority})"

    def is_retryable(self) -> bool:
        return self.attempts < MAX_RETRIES and not self.completed

    def mark_complete(self):
        self.completed = True
        logger.debug("Job %s marked complete after %d attempt(s)", self.job_id, self.attempts)

    def mark_failed(self, reason: str):
        self.error = reason
        logger.warning("Job %s failed (attempt %d): %s", self.job_id, self.attempts, reason)


# ── Queue ─────────────────────────────────────────────────────────────────────

class JobQueue:
    """Priority queue backed by a plain list. Not thread-safe."""

    def __init__(self):
        self._jobs: list[Job] = []
        self._handlers: dict[str, Callable] = {}
        self._stats: dict[str, int] = defaultdict(int)

    # Registration

    def register(self, task_name: str, handler: Callable):
        """Bind a handler function to a task name."""
        if task_name in self._handlers:
            logger.warning("Overwriting existing handler for task '%s'", task_name)
        self._handlers[task_name] = handler

    # Submission

    def submit(self, job: Job):
        """Add a job to the queue, maintaining priority order."""
        self._jobs.append(job)
        self._jobs.sort(key=lambda j: (j.priority, j.created_at))
        self._stats["submitted"] += 1
        logger.info("Queued %r  (queue depth: %d)", job, len(self._jobs))

    def submit_many(self, jobs: list[Job]):
        for job in jobs:
            self.submit(job)

    # Dispatch

    def process_next(self) -> bool:
        """Pop and execute the highest-priority job. Returns True if a job was run."""
        pending = [j for j in self._jobs if not j.completed]
        if not pending:
            return False

        job = pending[0]
        self._jobs.remove(job)
        self._dispatch(job)
        return True

    def drain(self):
        """Process all queued jobs in priority order."""
        processed = 0
        while self.process_next():
            processed += 1
        logger.info("Queue drained — %d job(s) processed", processed)

    def _dispatch(self, job: Job):
        handler = self._handlers.get(job.task)
        if handler is None:
            job.mark_failed(f"no handler registered for task '{job.task}'")
            self._stats["failed"] += 1
            return

        job.attempts += 1
        try:
            handler(job.payload)
            job.mark_complete()
            self._stats["completed"] += 1
        except Exception as exc:
            job.mark_failed(str(exc))
            self._stats["failed"] += 1
            if job.is_retryable():
                logger.info("Requeueing %r for retry", job)
                self.submit(job)

    # Introspection

    def depth(self) -> int:
        return len([j for j in self._jobs if not j.completed])

    def stats(self) -> dict:
        return dict(self._stats)

    def find(self, job_id: str) -> Optional[Job]:
        for job in self._jobs:
            if job.job_id == job_id:
                return job
        return None


# ── Helpers ───────────────────────────────────────────────────────────────────

def build_job(task: str, payload: dict, priority: int = PRIORITY_NORMAL) -> Job:
    """Create a Job with an auto-generated ID based on the current timestamp."""
    job_id = f"{task}-{int(time.time() * 1000)}"
    return Job(job_id=job_id, task=task, payload=payload, priority=priority)


def retry_failed(queue: JobQueue, jobs: list[Job]):
    """Re-submit any failed jobs that still have retry attempts remaining."""
    requeued = 0
    for job in jobs:
        if job.error and job.is_retryable():
            job.error = None  # clear previous error before retrying
            queue.submit(job)
            requeued += 1
    if requeued:
        logger.info("Re-submitted %d failed job(s)", requeued)


def summarize(jobs: list[Job]) -> dict[str, Any]:
    """Return a summary dict grouping jobs by completion status."""
    completed = [j for j in jobs if j.completed]
    failed    = [j for j in jobs if j.error and not j.completed]
    pending   = [j for j in jobs if not j.completed and not j.error]

    return {
        "total":     len(jobs),
        "completed": len(completed),
        "failed":    len(failed),
        "pending":   len(pending),
        "avg_attempts": (
            sum(j.attempts for j in completed) / len(completed)
            if completed else 0.0
        ),
    }


# ── Example handlers ──────────────────────────────────────────────────────────

def handle_email(payload: dict):
    recipient = payload.get("to", "unknown")
    subject   = payload.get("subject", "(no subject)")
    logger.info("Sending email to %s: %s", recipient, subject)
    # simulate occasional failure for testing retry logic
    if not recipient or "@" not in recipient:
        raise ValueError(f"invalid recipient address: {recipient!r}")


def handle_report(payload: dict):
    report_type = payload["type"]
    date_range  = payload.get("range", "last_30_days")
    logger.info("Generating %s report for range '%s'", report_type, date_range)


def handle_cleanup(payload: dict):
    older_than = payload.get("older_than_days", 90)
    dry_run    = payload.get("dry_run", True)
    action = "would delete" if dry_run else "deleting"
    logger.info("%s records older than %d days", action, older_than)


# ── Entry point ───────────────────────────────────────────────────────────────

def main():
    logging.basicConfig(level=logging.INFO, format="%(levelname)s  %(message)s")

    q = JobQueue()
    q.register("send_email",    handle_email)
    q.register("generate_report", handle_report)
    q.register("cleanup",       handle_cleanup)

    jobs = [
        build_job("send_email",       {"to": "alice@example.com", "subject": "Weekly digest"}, PRIORITY_HIGH),
        build_job("send_email",       {"to": "bob@example.com",   "subject": "Invoice #4421"}, PRIORITY_NORMAL),
        build_job("send_email",       {"to": "bad-address",       "subject": "Should fail"},   PRIORITY_NORMAL),
        build_job("generate_report",  {"type": "sales", "range": "last_7_days"},               PRIORITY_LOW),
        build_job("cleanup",          {"older_than_days": 60, "dry_run": False},               PRIORITY_LOW),
        build_job("unknown_task",     {},                                                       PRIORITY_NORMAL),
    ]

    q.submit_many(jobs)
    print(f"\nQueue depth before drain: {q.depth()}")

    q.drain()

    print("\n--- Summary ---")
    for key, val in summarize(jobs).items():
        print(f"  {key:<16} {val}")


if __name__ == "__main__":
    main()
