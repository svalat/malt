/***********************************************************
*    PROJECT  : MALT (MALoc Tracker)
*    DATE     : 02/2026
*    LICENSE  : CeCILL-C
*    FILE     : src/libinstrum/portability/OSUnix.hpp
*-----------------------------------------------------------
*    AUTHOR   : Sébastien Valat (ECR) - 2014
*    AUTHOR   : Sébastien Valat - 2015 - 2026
*    AUTHOR   : Sébastien Valat (INRIA) - 2025
***********************************************************/

#ifndef MALT_OS_UNIX_HPP
#define MALT_OS_UNIX_HPP

/**********************************************************/
//standard
#include <vector>
#include <string>
#include <cstdlib>
#include <iostream>

/**********************************************************/
namespace MALT
{

/**********************************************************/
struct LinuxInternalStatm
{
	size_t size{0};
	size_t resident{0};
	size_t share{0};
	size_t text{0};
	size_t lib{0};
	size_t data{0};
	size_t dirty{0};
};

/**********************************************************/
struct OSProcMemUsage
{
	size_t virtualMemory{0};
	size_t physicalMemory{0};
};

/**********************************************************/
struct OSMemUsage
{
	size_t totalMemory{0};
	size_t freeMemory{0};
	size_t buffers{0};
	size_t cached{0};
	size_t swap{0};
	size_t totalSwap{0};
	size_t directMap4K{0};
	size_t directMap2M{0};
};

/**********************************************************/
typedef std::vector<std::string> OSCmdLine;

/**********************************************************/
class OSUnix
{
	public:
		static OSProcMemUsage getProcMemoryUsage(void);
		static OSMemUsage getMemoryUsage(void);
		static unsigned int getPID(void);
		static std::string getExeName(void);
		static std::string getHostname(void);
		static std::string getDateTime(void);
		static OSCmdLine getCmdLine(void);
		static std::string getSignalName(int sig);
		static void printAvailSigs(std::ostream & out = std::cout);
		static int getSignalFromName(const std::string & signame);
		static void setSigHandler(void (*handler)(int s), const std::string & signame);
		static void setSigHandler(void (*handler)(int s), int sigid);
		static std::string loadTextFile(const std::string & file);
		static void * mmap(size_t size, bool populate = false);
		static void munmap(void * ptr,size_t size);
		static size_t getASLROffset(void * instrAddr);
		static bool hasASLREnabled(void);
		static size_t getFileSize(const std::string & fname);
};

}

#endif //MALT_OS_UNIX_HPP
