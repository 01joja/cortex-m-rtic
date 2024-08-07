
echo $1
echo $2

#v1=examples_report_passes_usage/*
#v2=report_code/passes

v1=$1
v2=$2

cp $v1* $v2

cd $v2


for file in *.rs; do
    # get line where rtic starts
    echo $file
    start=$(grep -n rtic::app $file | cut -f1 -d:)
    echo $start
    tail +$start $file > "$file.tmp"
    cp "$file.tmp" "${file%.rs}.tex"
    rm "$file.tmp"
    rm "$file"
done
