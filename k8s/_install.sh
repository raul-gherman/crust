for FILE in $(ls .)
do
    echo $FILE;
    #kubectl delete -f $FILE;
    kubectl apply -f $FILE;
done