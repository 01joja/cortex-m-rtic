
# echo "copy from "$1/$2\*
# echo "move to "$3/$4/

code=examples/report_test*
outputFolder=outputTemp
#v2=report_code/passes

rm $outputFolder -r
mkdir $outputFolder


for file in $code; do
    # get line where rtic starts
    
    if [[ "$file" == *'M'* ]]; then
        continue
    fi

    file=${file#"examples/"}
    file=${file%".rs"}
    fileM=$file"M"

    output=$outputFolder/$file.txt
    outputM=$outputFolder/$fileM.txt
    

    echo compiling and running $file
    cargo run --example $file &> $output
    
    echo compiling and running $fileM
    cargo run --example $fileM &> $outputM
    
done

echo 
cd $outputFolder

for file in *; do


    if [[ "$file" == *'M'* ]]; then
        continue
    fi

    file=${file#"examples/"}
    fileM=${file%".txt"}
    fileM=$fileM"M.txt"

    # echo $file
    # echo $fileM

    
    error=$(grep error $file)
    errorM=$(grep error $fileM)

    if [[ "$error" == *'error'* ]]; then
        echo
        echo failed to run ${file%".txt"}
        #lazy
        if [[ "$errorM" == *'error'* ]]; then
            echo failed to run ${fileM%".txt"}
        fi
        continue
    fi

    if [[ "$errorM" == *'error'* ]]; then
    
        echo
        echo failed to run ${fileM%".txt"}
        continue
    fi
    

    start=$(grep -n Running $file | cut -f1 -d:)
    start=$(echo "$(($start + 1))")
    info=$(tail +$start $file)

    start=$(grep -n Running $fileM | cut -f1 -d:)
    start=$(echo "$(($start + 1))")
    infoM=$(tail +$start $fileM)

    if [[ "$info" == "$infoM" ]]; then
        continue
    fi

    echo
    echo app:
    echo ${file%".txt"}
    echo and app:
    echo ${fileM%".txt"}
    echo did not have the same output

done
