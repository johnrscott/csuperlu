all: superlu.out dlinsol.out

superlu.out: superlu.c
	gcc superlu.c -lsuperlu -o $@

dlinsol.out: dlinsol.c
	gcc dlinsol.c -lsuperlu -o $@

