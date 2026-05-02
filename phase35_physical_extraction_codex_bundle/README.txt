Phase 35 Physical Behavior Extraction Codex Bundle

Purpose:
  Guidance files for Phase 35+ physical behavior extraction.

Key idea:
  Do not move all physical behavior at once.
  Phase 35 creates the physical extraction plan, guardrails, checks, and the first Storage State IO seam/scaffold.

Files included:
  codex_prompt_phase35_physical_extraction.md
  AGENTS_phase35_addendum.md
  plans_phase35_physical_extraction.md

  docs/phase35/PHASE35_PHYSICAL_EXTRACTION_PLAN.md
  docs/phase35/PHASE35_STORAGE_STATE_IO_SEAM.md
  docs/phase35/PHASE35_ACCEPTANCE.md
  docs/phase35/PHASE35_RISK_REGISTER.md
  docs/phase35/PHASE35_NEXT_PHASES.md

  scripts/check_phase35_physical_extraction_plan.sh
  scripts/check_phase35_storage_state_io_seam.sh
  scripts/check_phase35_no_hardware_regression.sh
  scripts/check_imported_reader_runtime_sync_phase35.sh
  scripts/revert_phase35_storage_state_io_seam.sh
  scripts/install_phase35_guidance_files.sh

Install:
  unzip phase35_physical_extraction_codex_bundle.zip
  cd phase35_physical_extraction_codex_bundle
  chmod +x scripts/*.sh
  ./scripts/install_phase35_guidance_files.sh /home/mindseye73/Documents/projects/vaachak-os

Then give codex_prompt_phase35_physical_extraction.md to Codex.
