a.out: a.o main.o sub.o 
	gcc a.o main.o sub.o -o a.out

a.o: a.c b.h a.h 
	gcc -c a.c -o a.o

main.o: main.c sub.h subsub.h 
	gcc -c main.c -o main.o

sub.o: sub.c sub.h subsub.h 
	gcc -c sub.c -o sub.o

run: a.out
	./a.out

clean:
	rm -f *.o *.out
