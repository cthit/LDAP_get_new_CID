#!/bin/bash
while read -r cid
do
	res=$(ldapsearch -z 0 -x -H ldap://ldap.chalmers.se -b "ou=groups,dc=chalmers,dc=se" "(&(cn=s_pdb_cat_admitted_program_student)(memberUid=$cid))")
	numResponses=$(echo "$res" | grep "^# numResponses: " | sed -n "s/^# numResponses: //p")
	if [[ $numResponses == "2" ]]
	then
		echo $cid
	fi
done < "$1"
