rm -r report_code/*
mkdir report_code/passes/
./renamefiles.sh examples_report_passes_usage/ report_code/passes/
mkdir report_code/rtic/
./renamefiles.sh examples/report_rtic report_code/rtic/
mkdir report_code/impl/
./renamefiles.sh examples/report_rtic report_code/impl/
