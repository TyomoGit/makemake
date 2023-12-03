a.out: main.o sub.o 
	gcc main.o sub.o -o a.out

main.o: main.c sub.h 
	gcc -c main.c -o main.o

sub.o: sub.c sub.h 
	gcc -c sub.c -o sub.o

run: a.out
	./a.out

clean:
	rm -f *.o *.out
