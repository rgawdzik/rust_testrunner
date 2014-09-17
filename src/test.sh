#Dynamic diff syntax:
#diff <(actual command) <(output expected)

#Assignment n Question 1 tests:
diff <(egrep 'hi') <(cat fixtures/mytest.txt) #=> (nothing) || error
diff <(egrep 'expression' fixtures/input.txt) <(cat fixtures/mytest.txt) #=> Either (nothing) || error
diff <(egrep 'expression' fixtures/input.txt) <(echo "should match") #=> (nothing) || error
egrep "hi" myfile.txt #=> should not match
diff <(./test < input) <(echo 0) #=> should output 0
