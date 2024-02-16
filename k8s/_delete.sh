for FILE in $(ls *.yaml)
do
    echo $FILE;
    kubectl delete -f $FILE;
done