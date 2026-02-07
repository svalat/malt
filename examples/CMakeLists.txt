############################################################
#    PROJECT  : MALT (MALoc Tracker)
#    DATE     : 01/2026
#    LICENSE  : CeCILL-C
#    FILE     : CMakeLists.txt
#-----------------------------------------------------------
#    AUTHOR   : Sébastien Valat - 2026
############################################################

############################################################
# install cpp example files
file(GLOB cpp_example_sources cpp/*.cpp)
install(FILES cpp/Makefile ${cpp_example_sources}
        DESTINATION share/malt/examples/cpp
)

############################################################
# install cpp example files
file(GLOB py_example_sources python/*.c python/*.h python/*.py)
install(FILES python/Makefile ${py_example_sources}
        DESTINATION share/malt/examples/python
)

############################################################
# install fortran example files
install(FILES fortran/01-ifort-heap-arrays/README.md
              fortran/01-ifort-heap-arrays/Makefile
              fortran/01-ifort-heap-arrays/main.f90
        DESTINATION share/malt/examples/fortran/01-ifort-heap-arrays
)
