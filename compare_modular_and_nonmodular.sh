
# echo "copy from "$1/$2\*
# echo "move to "$3/$4/

code=examples/report_test*
outputFolder=outputTemp
output=$outputFolder/output.txt
outputModular=$outputFolder/outputModular.txt
#v2=report_code/passes

echo $outputFolder
echo $output
echo $outputModular


mkdir $outputFolder
touch $output
touch $outputModular

rmdir $outputFolder -r
# mkdir $outputFolder

exit 1

for file in $code; do
    # get line where rtic starts
    
    # echo $file
    file=${file#"examples/"}
    file=${file%".rs"}
    echo $file

    cargo run --quiet --example $file 
    
    # start=$(grep -n rtic::app $file | cut -f1 -d:)
    # tail +$start $file > "$file.tmp"
    # output=${file#"$prefix"}
    # cp "$file.tmp" "${output%.rs}.tex"
    # rm "$file.tmp"
    # rm "$file"
done

rmdir $outputFolder -f
