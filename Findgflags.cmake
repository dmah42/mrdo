# - Try to find gflags
# Once done, this will define
#
#  GFLAGS_FOUND - system has GFLAGS
#  GFLAGS_INCLUDE_DIRS - the GFLAGS include directories
#  GFLAGS_LIBRARIES - link these to use GFLAGS

include(LibFindMacros)

# Use pkg-config to get hints about paths
libfind_pkg_check_modules(GFLAGS_PKGCONF gflags)

# Include dir
find_path(GFLAGS_INCLUDE_DIR
          NAMES gflags/gflags.h
          PATHS ${GFLAGS_PKGCONF_INCLUDE_DIRS}
)

# Finally the library itself
find_library(GFLAGS_LIBRARY
             NAMES gflags
             PATHS ${GFLAGS_PKGCONF_LIBRARY_DIRS}
)

# Set the include dir variables and the libraries and let libfind_process do the rest.
set(GFLAGS_PROCESS_INCLUDES GFLAGS_INCLUDE_DIR)
set(GFLAGS_PROCESS_LIBS GFLAGS_LIBRARY)
libfind_process(GFLAGS)

