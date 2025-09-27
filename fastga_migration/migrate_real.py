#!/usr/bin/env python3
"""
REAL migration tracker for FastGA C to Rust conversion.
Tracks the actual FastGA functions, not just utility functions.
"""

import json
import subprocess
import sys
import os
from typing import Dict, List
from datetime import datetime
from pathlib import Path

class RealMigrationTracker:
    def __init__(self):
        self.config_file = Path("real_migration_status.json")

        # The REAL functions we need to migrate
        self.function_groups = {
            "gene_core": [
                "fastga_malloc",
                "fastga_realloc",
                "fastga_strdup",
                "fastga_fopen",
                "fastga_path_to",
                "fastga_root",
                "fastga_compress_read",
                "fastga_uncompress_read",
                "fastga_number_read",
            ],
            "gdb": [
                "fastga_create_gdb",
                "fastga_free_gdb",
                "fastga_open_gdb",
                "fastga_close_gdb",
                "fastga_get_contig_seq",
                "fastga_load_sequences",
            ],
            "alignment": [
                "fastga_new_work_data",
                "fastga_free_work_data",
                "fastga_new_align_spec",
                "fastga_free_align_spec",
                "fastga_local_alignment",
                "fastga_compute_trace",
            ],
            "index": [
                "fastga_build_index",
                "fastga_free_index",
                "fastga_save_index",
                "fastga_load_index",
            ],
            "merge": [
                "fastga_find_matches",
                "fastga_free_match_list",
            ],
            "output": [
                "fastga_write_paf",
                "fastga_write_psl",
            ],
            "pipeline": [
                "fastga_run_alignment",
            ]
        }

        self.all_functions = []
        for group in self.function_groups.values():
            self.all_functions.extend(group)

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
                "last_updated": None,
                "utility_functions_done": True,  # The 11 utility functions we already did
            }

    def save_status(self):
        """Save migration status to file."""
        self.status["last_updated"] = datetime.now().isoformat()
        with open(self.config_file, 'w') as f:
            json.dump(self.status, f, indent=2)

    def progress(self):
        """Show migration progress."""
        total = len(self.all_functions)
        migrated = len(self.status["migrated"])
        tested = len(self.status["tested"])
        failed = len(self.status["failed"])

        print(f"\n{'='*70}")
        print("REAL FastGA Migration Progress")
        print(f"{'='*70}")
        print(f"Total REAL functions: {total}")
        print(f"Migrated:            {migrated} ({100*migrated/total:.1f}%)")
        print(f"Tested:              {tested} ({100*tested/total:.1f}%)")
        print(f"Failed:              {failed}")
        if self.status["in_progress"]:
            print(f"In Progress:         {self.status['in_progress']}")

        print(f"\n{'='*70}")
        print("Progress by Component:")
        print(f"{'='*70}")

        for group_name, functions in self.function_groups.items():
            group_migrated = sum(1 for f in functions if f in self.status["migrated"])
            group_total = len(functions)
            pct = 100 * group_migrated / group_total if group_total > 0 else 0
            status_bar = "█" * int(pct/10) + "░" * (10 - int(pct/10))
            print(f"{group_name:<12} [{status_bar}] {group_migrated}/{group_total} ({pct:.0f}%)")

        print(f"\n{'='*70}")
        print("Detailed Function Status:")
        print(f"{'='*70}")

        for group_name, functions in self.function_groups.items():
            print(f"\n{group_name.upper()}:")
            for func in functions:
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
                print(f"  {func:<35} {status}")

        if self.status["last_updated"]:
            print(f"\nLast updated: {self.status['last_updated']}")

    def estimate_lines(self):
        """Estimate lines of code for each component."""
        estimates = {
            "gene_core": 503,
            "gdb": 1974,
            "alignment": 6050,
            "index": 2000,
            "merge": 1500,
            "output": 1000,
            "pipeline": 2000,
        }

        total_lines = sum(estimates.values())
        migrated_lines = 0

        for group_name, functions in self.function_groups.items():
            group_migrated = sum(1 for f in functions if f in self.status["migrated"])
            group_total = len(functions)
            if group_total > 0:
                migrated_lines += estimates[group_name] * group_migrated / group_total

        print(f"\n{'='*70}")
        print("Estimated Lines of Code:")
        print(f"{'='*70}")
        print(f"Total:    ~{total_lines:,} lines")
        print(f"Migrated: ~{int(migrated_lines):,} lines ({100*migrated_lines/total_lines:.1f}%)")
        print(f"Remaining: ~{int(total_lines - migrated_lines):,} lines")

def main():
    tracker = RealMigrationTracker()

    if len(sys.argv) < 2:
        print("Usage: python migrate_real.py [command]")
        print("Commands:")
        print("  progress  - Show REAL migration progress")
        print("  estimate  - Show lines of code estimates")
        sys.exit(1)

    command = sys.argv[1]

    if command == "progress":
        tracker.progress()
    elif command == "estimate":
        tracker.progress()
        tracker.estimate_lines()
    else:
        print(f"Unknown command: {command}")
        sys.exit(1)

if __name__ == "__main__":
    main()