main.out: main.o sub.o 

main.o: main.c sub.h 
	gcc -c main.c -o main.o

sub.o: sub.c sub.h 
	gcc -c sub.c -o sub.o

run: main.out
	./main.out

clean:
	rm -f *.o *.out
