#!/usr/bin/env python3
"""
Migration tracker for FASTGA C to Rust conversion.
Tracks which functions have been migrated and validates correctness.
"""

import json
import subprocess
import sys
import os
from typing import Dict, List, Set
from datetime import datetime
from pathlib import Path

class MigrationTracker:
    def __init__(self):
        self.config_file = Path("migration_status.json")
        self.functions = [
            "encode_2bit",
            "decode_2bit",
            "encode_kmer",
            "decode_kmer",
            "kmer_reverse_complement",
            "hash_kmer",
            "score_match",
            "edit_distance",
            "gc_content",
            "count_bases",
            "reverse_complement_seq",
        ]
        self.status = self.load_status()

    def load_status(self) -> Dict:
        """Load migration status from file."""
        if self.config_file.exists():
            with open(self.config_file, 'r') as f:
                return json.load(f)
        else:
            return {
                "migrated": [],
                "tested": [],
                "failed": [],
                "in_progress": None,
                "last_updated": None
            }

    def save_status(self):
        """Save migration status to file."""
        self.status["last_updated"] = datetime.now().isoformat()
        with open(self.config_file, 'w') as f:
            json.dump(self.status, f, indent=2)

    def test_function(self, name: str) -> bool:
        """Run tests for a specific function."""
        print(f"Testing {name}...")

        # Run Rust tests for this function
        result = subprocess.run(
            ["cargo", "test", f"test_{name}", "--", "--nocapture"],
            cwd="rust_core",
            capture_output=True,
            text=True
        )

        if result.returncode == 0:
            print(f"  ✅ {name} tests passed")
            return True
        else:
            print(f"  ❌ {name} tests failed")
            print(result.stdout)
            print(result.stderr)
            return False

    def migrate_function(self, name: str) -> bool:
        """Mark a function as using Rust implementation."""
        print(f"\n{'='*60}")
        print(f"Migrating: {name}")
        print(f"{'='*60}")

        self.status["in_progress"] = name
        self.save_status()

        # Test the function
        if not self.test_function(name):
            self.status["failed"].append(name)
            self.status["in_progress"] = None
            self.save_status()
            return False

        # Update config to use Rust version
        config_update = f"""
Updating Config to use Rust {name}:
  use_rust_{name}: false -> true
"""
        print(config_update)

        # Mark as migrated
        if name not in self.status["migrated"]:
            self.status["migrated"].append(name)
        if name not in self.status["tested"]:
            self.status["tested"].append(name)
        if name in self.status["failed"]:
            self.status["failed"].remove(name)
        self.status["in_progress"] = None
        self.save_status()

        print(f"  ✅ Successfully migrated {name}")
        return True

    def run_all_tests(self) -> bool:
        """Run all tests."""
        print("\nRunning all tests...")

        result = subprocess.run(
            ["cargo", "test"],
            cwd="rust_core",
            capture_output=True,
            text=True
        )

        if result.returncode == 0:
            print("  ✅ All tests passed")
            return True
        else:
            print("  ❌ Some tests failed")
            print(result.stdout)
            print(result.stderr)
            return False

    def build_c_library(self) -> bool:
        """Build the C library."""
        print("Building C library...")

        result = subprocess.run(
            ["make", "clean", "all"],
            cwd="c_core",
            capture_output=True,
            text=True
        )

        if result.returncode == 0:
            print("  ✅ C library built successfully")
            return True
        else:
            print("  ❌ Failed to build C library")
            print(result.stdout)
            print(result.stderr)
            return False

    def progress(self):
        """Show migration progress."""
        total = len(self.functions)
        migrated = len(self.status["migrated"])
        tested = len(self.status["tested"])
        failed = len(self.status["failed"])

        print(f"\n{'='*60}")
        print("FASTGA Migration Progress")
        print(f"{'='*60}")
        print(f"Total functions: {total}")
        print(f"Migrated:       {migrated} ({100*migrated/total:.1f}%)")
        print(f"Tested:         {tested} ({100*tested/total:.1f}%)")
        print(f"Failed:         {failed}")
        if self.status["in_progress"]:
            print(f"In Progress:    {self.status['in_progress']}")

        print(f"\n{'Function':<30} {'Status':<20}")
        print("-" * 50)
        for func in self.functions:
            if func in self.status["migrated"]:
                status = "✅ Migrated"
            elif func in self.status["failed"]:
                status = "❌ Failed"
            elif func == self.status["in_progress"]:
                status = "🔄 In Progress"
            elif func in self.status["tested"]:
                status = "🧪 Tested (C only)"
            else:
                status = "⏳ Pending"
            print(f"{func:<30} {status:<20}")

        if self.status["last_updated"]:
            print(f"\nLast updated: {self.status['last_updated']}")

    def reset(self):
        """Reset migration status."""
        self.status = {
            "migrated": [],
            "tested": [],
            "failed": [],
            "in_progress": None,
            "last_updated": None
        }
        self.save_status()
        print("Migration status reset")

    def next_function(self) -> str:
        """Get next function to migrate."""
        for func in self.functions:
            if func not in self.status["migrated"] and func not in self.status["failed"]:
                return func
        return None

def main():
    tracker = MigrationTracker()

    if len(sys.argv) < 2:
        print("Usage: python migrate.py [command]")
        print("Commands:")
        print("  build     - Build C library")
        print("  test      - Run all tests")
        print("  progress  - Show migration progress")
        print("  migrate [function] - Migrate specific function")
        print("  next      - Migrate next function")
        print("  reset     - Reset migration status")
        sys.exit(1)

    command = sys.argv[1]

    if command == "build":
        tracker.build_c_library()
    elif command == "test":
        tracker.run_all_tests()
    elif command == "progress":
        tracker.progress()
    elif command == "migrate":
        if len(sys.argv) < 3:
            print("Please specify a function to migrate")
            sys.exit(1)
        func = sys.argv[2]
        if func not in tracker.functions:
            print(f"Unknown function: {func}")
            print(f"Available: {', '.join(tracker.functions)}")
            sys.exit(1)
        tracker.migrate_function(func)
    elif command == "next":
        func = tracker.next_function()
        if func:
            print(f"Next function to migrate: {func}")
            tracker.migrate_function(func)
        else:
            print("All functions have been migrated or failed!")
    elif command == "reset":
        tracker.reset()
    else:
        print(f"Unknown command: {command}")
        sys.exit(1)

if __name__ == "__main__":
    main()