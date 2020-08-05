# That's Just Math  
**Starts:** 2020-07-01
**Ends:** 2020-08-11  
  
I don't like doing math. Instead, I give all of my logic to the machine computer and save none of it for myself. I also don't like having to leave the terminal, so if I could do my math there too, that'd be ideal.  
  
It'd be neat if, given a string of math, we could also be given the output. So, given the string `1+1` you should be able to print out `2`.  
  
```bash  
$ ./program 1+1  
2  
```  
  
Next, consider multiplication and division. From elementary math, you should remember that multiplication and division come before addition and subtraction. So `1+1*2` would return `3`.  
```bash  
$ ./program 1+1*2  
3  
```  
  
Suppose we wanted to do the `1+1` first. We could surround `1+1` with parentheses.  
```bash  
$ ./program (1+1)*2  
4  
```  
  
These are simple examples, but something like `2*8-12/(2+4)` would return `14`.  
  
The requirements for this challenge are as follows:  
  
```  
As someone that can't do quick maths,  
I would like to be able to put my equation into a program that does the maths quick  
so that I don't have to do the maths  
```  
  
## Assumptions:  
1. You don't have to worry about spaces. For example, always expect `1+1` instead of `1 + 1`.  
2. Your program will be a command line application. However that manifests is up to you, but it should be runnable from the terminal.  
3. You can expect simple addition, subtraction, multiplication, and division.  
4. It is possible for parentheses to be nested like this: `1+(3*(1+2))-8`.  
  
## Bonus Challenge:  
Deploy your application for other people to use.

[Powered by FordLabs](https://fordlabs.com/)
