DEPDIR := .deps

CPPFLAGS=-isystem /home/pseyfert/coding/explore-rangev3/install-range-v3/include

all: main

### NVC++
# CXX=/opt/nvidia/hpc_sdk/Linux_x86_64/2022/compilers/bin/nvc++
# TARGET_ARCH=-gpu=ccnative --c++20 -acc=gpu -stdpar=gpu -Minfo=stdpar
# main: main.o parts.o

### G++
CXXFLAGS=-Wall -Wextra --std=c++2b -O3 -g -march=native
LDLIBS=-L/home/pseyfert/coding/explore-rangev3/install-range-v3/lib -ltbb
DEPFLAGS = -MT $@ -MMD -MP -MF $(DEPDIR)/$*.d
parts.o : CXXFLAGS += -fopt-info-vec-all
main: main.o


# common

main : CC = ${CXX}

# delete builtin. Otherwise %: $.cpp with (LINK.cpp) kicks in
% : %.cpp

%.o : %.cpp
%.o : %.cpp Makefile $(DEPDIR)/%.d | $(DEPDIR)
	$(COMPILE.cpp) $(OUTPUT_OPTION) $<

# copied from https://make.mad-scientist.net/papers/advanced-auto-dependency-generation/

SRCS  := $(wildcard *.cc *.cpp)

# copied from default `make -p`
COMPILE.cpp = $(CXX) $(DEPFLAGS) $(CXXFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -c

$(DEPDIR): ; @mkdir -p $@

DEPFILES := $(SRCS:%.cpp=$(DEPDIR)/%.d)
$(DEPFILES):
include $(wildcard $(DEPFILES))

clean:
	rm -f main *.o

.PHONY: clean all
