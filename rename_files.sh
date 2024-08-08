
echo "copy from "$1/$2\*
echo "move to "$3/$4/

#v1=examples_report_passes_usage/*
#v2=report_code/passes

origin=$(echo $1/$2*)
prefix=$2
path=$(echo $3/$4/)
cp $origin* $path
cd $path


for file in *.rs; do
    # get line where rtic starts
    start=$(grep -n rtic::app $file | cut -f1 -d:)
    tail +$start $file > "$file.tmp"
    output=${file#"$prefix"}
    cp "$file.tmp" "${output%.rs}.tex"
    rm "$file.tmp"
    rm "$file"
done
