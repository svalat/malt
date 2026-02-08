############################################################
#    PROJECT  : MALT (MALoc Tracker)
#    DATE     : 02/2026
#    LICENSE  : CeCILL-C
#    FILE     : examples/fortran/01-ifort-heap-arrays/Makefile
#-----------------------------------------------------------
#    AUTHOR   : Sébastien Valat (INRIA) - 2025
#    AUTHOR   : Sébastien Valat - 2026
############################################################

############################################################
all:
	ifx -O0 -g -finstrument-functions -heap-arrays -o reproducer_heap main.f90
	ifx -O0 -g -finstrument-functions -o reproducer_stack main.f90

############################################################
test:
	malt ./reproducer_heap
	malt ./reproducer_stack

############################################################
clean:
	rm -f reproducer_heap reproducer_stack
	rm -f malt-*

############################################################
.PHONY: all test clean