rm -r report_code/passes
mkdir report_code/passes/
./rename_files.sh examples_report_passes_usage report_passes_ report_code passes
rm -r report_code/rtic
mkdir report_code/rtic/
./rename_files.sh examples report_rtic_ report_code rtic
rm -r report_code/impl
mkdir report_code/impl/
./rename_files.sh examples report_impl_ report_code impl
rm -r report_code/codegen
mkdir report_code/codegen/

cp examples_report_codegen/cp_* report_code/codegen
cd report_code/codegen

for file in *.rs; do
    # get line where rtic starts
    output=${file#"cp_"}
    cp "$file" "${output%.rs}.tex"
    rm "$file"
done
