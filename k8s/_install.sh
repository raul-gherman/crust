for FILE in $(ls *.yaml)
do
    echo $FILE;
    kubectl apply -f $FILE;
done