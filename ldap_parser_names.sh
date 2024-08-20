#!/bin/bash
while IFS=, read -r col1 col2
do
    res=$(ldapsearch -z 0 -x -H ldap://ldap.chalmers.se -b "ou=people,dc=chalmers,dc=se" "(cn=$col1 $col2)")
    count=$(echo "$res" | grep "^# numEntries: " | sed "s/^# numEntries: //")
    if [[ $count == "1" ]]
    then
        uid=$(echo "$res" | grep '^uid:*' | sed -n 's/^uid: //p')
	echo $uid
    fi
done < "$1"
