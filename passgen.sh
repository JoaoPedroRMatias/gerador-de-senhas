#!/bin/bash

array=("A" "a" "B" "b" "C" "c" "D" "d" "E" "e" "F" "f" "G" "g" "H" "h" "I" "i" "J" "j" "K" "k" "L" "l" "M" "m" "N" "n" "O" "o" "P" "p" "Q" "q" "R" "r" "S" "s" "T" "t" "U" "u" "V" "v" "W" "w" "X" "x" "Y" "y" "Z" "z" "0" "1" "2" "3" "4" "5" "6" "7" "8" "9")

array_special=("#" "$" "%" "&" "@" "!" "?" "*" "+" "-" "_")

password=

while true; do
    read -p "Digite o tamanho que deseja a senha: " pass_len

    if [[ $pass_len =~ ^[0-9]+$ && $pass_len -gt 10 ]]; then
        break
    else
        echo "Entrada inválida. Deve ser um NUMERO e ACIMA DE 10 caracteres."
    fi
done

pass_len=$pass_len/2

for ((i=0; i<$pass_len; i++));
do
	random_number=$((RANDOM % ${#array[@]}))
	letter=${array[$random_number]}
	
	random_special=$((RANDOM % ${#array_special[@]}))
	special=${array_special[$random_special]}

	password+=${letter}
	password+=${special}
done

echo 
echo "Senha:" $password
echo 
