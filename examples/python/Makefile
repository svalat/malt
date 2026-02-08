############################################################
#    PROJECT  : MALT (MALoc Tracker)
#    DATE     : 02/2026
#    LICENSE  : CeCILL-C
#    FILE     : examples/python/Makefile
#-----------------------------------------------------------
#    AUTHOR   : Sébastien Valat (INRIA) - 2025
#    AUTHOR   : Sébastien Valat - 2026
############################################################

############################################################
all:
	gcc -g -fPIC -shared step-3-calling-c-func.c -o libstep-3-calling-c-func.so

############################################################
malt:
	malt-python ./step-1-hello-world.py
	malt-python ./step-2-fill-list.py
	malt-python ./step-3-calling-c-func.py
	malt-python ./step-4-using-numpy.py

############################################################
clean:
	rm -f malt-*.json
	rm -f libstep-3-calling-c-func.so

############################################################
.PHONY: clean
