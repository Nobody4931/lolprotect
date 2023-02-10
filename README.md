# lolprotect
some basic file encryption thing

## how secure is it?
it uses xor encryption how secure do you think its gonna be

## aren't there better ways of doing this?
yes

## why'd you make this then?
i thought itd be pretty funny

also i wanted to spite my school for not allowing students to plug in
usbs to bring files from home even though they could just download
any files they wanted straight off the internet

## so how does it work?
it just strings the contents of all the files being encrypted together
and uses a bunch of sha512 hashes of the encryption password modified
by some state as the xor key

it also generates a file header and a checksum to ensure that the
correct version of the decryptor is being used and that the decryption
password is correct
