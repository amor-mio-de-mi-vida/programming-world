# Modern CMake

参考：[Introduction · Modern CMake (modern-cmake-cn.github.io)](https://modern-cmake-cn.github.io/Modern-CMake-zh_CN/)

[An Introduction to Modern CMake · Modern CMake (cliutils.gitlab.io)](https://cliutils.gitlab.io/modern-cmake/)

其他的资料：

本书原作者的其他资料:

- [HSF CMake Training](https://hsf-training.github.io/hsf-training-cmake-webpage/01-intro/index.html)
- [Interactive Modern CMake talk](https://gitlab.com/CLIUtils/modern-cmake-interactive-talk)

在网上还有一些其他的地方可以找到好的资讯。下面是其中的一些:

- [The official help](https://cmake.org/cmake/help/latest/): 非常棒的文档。组织得很好，有很好的搜索功能，而且你可以在顶部切换版本。它只是没有一个很好的 “最佳实践教程”，而这正是本书试图解决的内容。
- [Effective Modern CMake](https://gist.github.com/mbinna/c61dbb39bca0e4fb7d1f73b0d66a4fd1): 一个很好的 do's and don'ts 的清单。
- [Embracing Modern CMake](https://steveire.wordpress.com/2017/11/05/embracing-modern-cmake/): 一篇对术语有很好描述的文章。
- [It's time to do CMake Right](https://pabloariasal.github.io/2018/02/19/its-time-to-do-cmake-right/): 一些现代的 CMake 项目的最佳实践。
- [The Ultimate Guide to Modern CMake](https://rix0r.nl/blog/2015/08/13/cmake-guide/): 一篇有着本书类似目的稍显过时的文章。
- [More Modern CMake](https://youtu.be/y7ndUhdQuU8): 来自 Meeting C++ 2018 的一个很棒的演讲，推荐使用 CMake 3.12 以上版本。该演讲将 CMake 3.0+ 称为 “现代 CMake”，将 CMake 3.12+ 称为 “更现代的 CMake”。
- [Oh No! More Modern CMake](https://www.youtube.com/watch?v=y9kSr5enrSk): More Modern CMake 的续篇。
- [toeb/moderncmake](https://github.com/toeb/moderncmake): 关于 CMake 3.5+ 的很好的介绍和例子，包括从语法到项目组织的介绍。

## An Introduction to Modern CMake

### Running a project



**building a project**

**Unless otherwise noted, you should always make a build directory and build from there**. You can technically do an in-source build, but you’ll have to be careful not to overwrite files or add them to git, so just don’t.

Here’s the Classic CMake Build Procedure(TM):

```bash
~/package $ mkdir build
~/packgae $ cd build
~/package/build $ cmake ..
~/package/build $ make
```

You can replace the make line with `cmake ––build`. if you’d like and it will call `make` or whatever build tool you are using.If you are using a newer version of CMake (which you usually should be, except for checking compatibility with older CMake), you can instead do this:

```bash
~/package $ cmake -S . -B build
~/package $ cmake --build build
```

Any one of these commands will install:

```bash
# From the build directory (pick one)

~/package/build $ make install
~/package/build $ cmake --build . --target install
~/package/build $ cmake --install . # CMake 3.15+ only

# From the source directory (pick one) 
~/package $ ~/package/build $ make -C build install
~/package/build $ cmake --build build --target install
~/package/build $ cmake --install build # CMake 3.15+ only
```

if you use `cmake –-build` instead of directly calling the underlying build system:

`-v` for verbose builds(CMake 3.14+)

`-j N` for parallel builds on N cores (CMake 3.12+)

`—-target` (any version of CMake) or `-t` (CMake 3.15+) to pick a target.

Otherwise these commands vary between build systems, such as `VERBOSE=1 make` and `ninja -v`.You can instead use the environment variables for these, as well, such as `CMAKE_BUILDE_PARALLEL_LEVEL` (CMake 3.12+) and `VERBOSE` (CMake 3/14+).

**Picking a compiler**

**Selecting a compiler must be done on the first run in an empty directory.**

```bash
~/package/build $ CC=clang CXX=clang++ cmake ..
```



**Picking a generator**

You can build with a variety of tools; `make` is usually the default. To see all the tools CMake knows about on your system, run

```bash
~/package/build $ cmake --help
```

And you can pick a tool with `-G"My Tool"` (quotes only needed if spaces are in the tool name). You should pick a tool on your first CMake call in a directory, just like the compiler. Feel free to have several build directories, like `build/` and `buildXcode`. you can set the environment variable `CMAKE_GENERATOR` to control the default generator. 

> Note : makefiles will only run in parallel if you explicitly pass a number of threads, such as `make -j2`, while Ninja will automatically run in parallel. You can directly pass a parallelization option such as `-j2` to the `cmake —-build .` command in recent versions of CMake.

**Setting options**

You set optinos in CMake with `-D`.

You can see a list of options with `-L`, or a list with human-readable help with `-LH`.

if you don’t list the source/build directory, the listing will not rerun CMake(`cmake -L` instead of `cmake -L .`)

**Verbose and partial builds**

You can build just a part of a build by specifying a target, such as the name of a library or executable you’ve defined in CMake, and make will just build that target.



**Options**

CMake has support for cached options. A Variable in CMake can be marked as “cached”, which means it will be written to the cache (a file called `CMakeCache.txt` in the build directory) when it is encountered. You can preset(or change) the value of a cached option on the command line with `-D`. When CMake looks for a cached variable, it will use the existing value and will not overwrite it.



**Standard options**

These are common CMake options to most packages:

- `-DCMAKE_BUILD_TYPE=` Pick from Release, RelWithDebInfo, Debug, or sometimes more.
- `-DCMAKE_INSTALL_PREFIX=` The location to install to . System install on UNIX would often be `/usr/local` (the default), user directories are often `~/local`, or you can pick a folder.
- `-DCMAKE_SHARED_LIBS=` You can set this `ON` or `OFF` to control the default for shared libraries (the author can pick one vs. the other explicitly instead of using the default, though)
- `-DBUILD_TESTING` This is a common name for enabling tests, not all packages use it, though, sometimes with good reason.



**Debugging your CMake files**

我们已经提到了在构建时可以有详细输出，但你也可以看到详细的 CMake 配置输出。`--trace` 选项能够打印出运行的 CMake 的每一行。由于它过于冗长，CMake 3.7 添加了 `--trace-source="filename"` 选项，这让你可以打印出你想看的特定文件运行时执行的每一行。如果你选择了要调试的文件的名称（在调试一个 CMakeLists.txt 时通常选择父目录，因为它们名字都一样），你就会只看到这个文件里运行的那些行。这很实用！



### Do’s and Don’ts



CMake Antipatterns

The next two lists are heavily based on the excellent gist [Effective Modern CMake](https://gist.github.com/mbinna/c61dbb39bca0e4fb7d1f73b0d66a4fd1). That list is much longer and more detailed, feel free to read it as well.

- **Do not use golbal functions:** This includes `link_directories`, `include_libraries`, and similar.
- **Don’t add unneeded PUBLIC requirements:** You should avoid forcing something on users that is not required(`-Wall`). Make these PRIVATE instead.
- **Don’t GLOB files:** Make or another tool willnot know if you add files without rerunning CMake.Note that CMake 3.12 adds a `CONFIGURE_DEPENDS` flag that makes this far better if you need to use it.
- **Link to built files directly:** Always link to targets if avaliable.
- **Never skip PUBLIC/PRIVATE when linking:** This causes all future linking to be keyword-less.



CMake Patterns

- **Treat CMake as code:** It is code. It should be as clean and readable as all other code.
- **Think in targets:** Your targets should represent concepts. Make an (IMPORTED) INTERFACE target for anything that should stay rtogether and link to that.
- **Export your interface:** You should be able to run from build or install.
- **Write a Config.cmake file:** This is what a library author should do to support clients.
- **Make ALAS targets to keep usage consistent:** Using `add_subdirectory` and `find_package` should provide the same targets and namespaces.
- Combine common functionality into clearly documented functions or macros: Functions are better usually.
- **Use lowercase function names:** CMake functions and macros can be called lower or upper case. Always use lower case. Upper case is for variables.
- **Use `cmake_policy` and/or range of versions:** Policies change for a reason. Only piecemeal set OLD policies if you have to .



## Introduction to the Basics

Here’s the first line of every `CMakeLists.txt`, which is the required name of the file CMake looks for:

```cmake
cmake_minimum_required(VERSION 3.1)
```

This is what new projects should do :

```cmake
cmake_minimum_required(VERSION 3.7...3.26)

if(${CMAKE_VERSION} VERSION LESS 3.12)
	cmake_policy(VERSION ${CMAKE_MAJOR_VERSION}.${CMAKE_MINOR_VERSION})
endif()
```

if you need to support non-command line Windows builds for older MSVC versions, you will want to do this instead:

```cmake
cmake_minimum_required(VERSION 3.7)

if(${CMAKE_VERSION} VERSION_LESS 3.26)
	cmake_policy(VERSION ${CMAKE_MAJOR_VERSION}.${CMAKE_MINOR_VERSION})
else()
	cmake_policy(VERSION 3.26)
endif()
```

> if you really need to set to a low value here, you can use `cmake_policy` to conditionally increase the policy level or set a specific policy. Pleaseat least do this for your macOS users!





Setting a project

Now, every top-level CMake file will have the next line:

```cmake
project(MyProject VERSION 1.0
				  DESCRIPTION "Very nice project"
				  LANGUAGES CXX)
```

All the keyword arguments here are optional. The version sets a bunch of variables, like`MyProject_VERSION` and 	`PROJECT_VERSION`. The languages are `C`, `CXX` ,`Fortran`, `ASM`, `CUDA`, `CShap`, and `SWIFT`. `C CXX` is the default.



Making an executable

```cmake
add_executable(one two.cpp three.h)
```

More about the general build system and targets is available at [buildsystem](https://cmake.org/cmake/help/latest/manual/cmake-buildsystem.7.html).



Making a library

```cmake
add_library(one STATIC two.cpp three.h)
```

You get to pick a type of library, STATIC, SHARED, or MOUDLE. If you leave this choice off, the value of `BUILD_SHARED_LIBS` will be used to pick between STATIC and SHARED.

You can also make an `ALIAS` library with an existing library, which simply gives you an new name for a target. The one benefit to this is that you can make libraries with`::` in the name (which you’ll see later).



Targets are your friend

Now we’ve specified a target, how do wee add information about it? For example. maybe it needs an include directory:

```cmake
target_include_directories(one PUBLIC include)
```

`target_include_directories` adds an include directory to a target. `PUBLIC` doesn’t mean much for an executable; for a libraryit lets CMake know that any targets that link to this target must also need that include directory. Other options are `PRIVATE`(only affect the current target, not dependencies) and `INTERFACE` (only needed for dependencies).

We can then chain targets:

```cmake
add_library(another STATIC another.cpp another.h)
target_link_libararies(another PUBLIC one)
```

`target_link_libraries` takes a target (`another`) and adds a dependency if a target is given. If no target of that name(`one`) exists, then it adds a link to a library called `one` on your path (hence the name of the command). Or you can give it a full path to a library. Or a linker flag. Classic CMake allowed you to skip the keyword selection of `PUBLIC`, etc. If this was done on a target, you’ll get an error if you try to mix styles further fown the chain.

Targets can have include directories, linked libraries(or libnked targets), compile options, compile definitions, compile features , and more, **you can often get targets(and always make targets) to represent all the libraries you use. Even things that are not true libararies, like  OpenMP, can be represented with targets.**



Dive in

```cmake
cmake_minimun_required(VERSION 3.8)

project(Calculator LAGUAGES CXX)

add_library(calclib STATIC src/calclib.cpp include/calc/lib.hpp)
target_include_directories(calclib PUBLIC include)
target_compile_features(calclib PUBLIC cxx_std_11)

add_executable(calc apps/calc.cpp)
target_link_libraries(calc PUBLIC calclib)
```

> The `::` syntax was originally intended for `INTERFACE IMPROTED` LIBRARIES, which were explicitly suppoesd to be libraries defined outisde the current project. But, because of this, most of the `target_*` commands don’t work on `IMPORTED` libraries, making them hard to set up yourself. So don’t use the `IMPROTED` keyword for now, and use an `ALIAS` target instead; it will be fine until you start exporting targets.



### Variables and the Cache



Local Variables

A local variable is set like this:

```cmake
set(MY_VARIABLE "value")
```

CMake has the concept of scope; you can access the value of the variable after yo set it as long as you are in the same scope. If you leave a function or a file in a sub directory, the variable will no longer to be defined. You can set a variable in the scope immediately above your current one with `PARENT_SCOPE` at the end.

Lists are simply a series of values when you set them:

```cmake
set(MY_LIST "one" "two")
```

which internally become `;` separated values. So this is an identical statement:

```cmake
set(MY_LIST "one;two")
```

The `list(` command has utilities for working with lists, and `separate_arguments` will turn a space separated string into a list(inplace). Note that an unquoted value in CMake is the same as a quoted on if there are no spaces in it; this allows you to skip the quotes most of the time when working with value that you know could not contain spaces.

When a variable is expanded using `${}` syntax, all the same rules about spaces apply. Be expecially carefl with paths; paths may contain a space at any time and should always be quoted when they are a variable(never write `${MY PATH}`, always should be `“${MY PATH}` ”).



Cache Variables 

if you want to set a variable from the command line, CMake offers a variable cache. Some variables are already here, like `CMAKE_BUILD_TYPE` .The syntax for eclaring a variable and setting it if it is not already set is:

```cmake
set(MY_CACHE_VARIABLE "VALUE" CACHE STRING "Description")
```

**This will not replace an existing value.** This is so that you can set these on the command line and not have them overridden when the CMake file executes. If you want to use these variables as a make-shift folbal variable, then you can do:

```cmake
set(MY_CACHE_VARIABLE "VALUE" CACAHE STRING "" FORCE)
mark_as_advanced(MY_CACHE_VARIABLE)
```

The first line will cause the value to be set no mater what, and the second line will keep the variable from showing up in the list of variables if you run `cmake =L ..` or use a GUI. This is so comon, you can also use the `INTERNAL` type to do the same thing(though technically it forces the STRING type, this won’t affect any CMake code that depends on the variable).

```cmake
set(MY_CACHE_VARIABLE "VALUE" CACHE INTERNAL "")
```

Since `BOOL` is such a common variable type, you can set it more succinctly with the shortcut:

```cmake
option(MY_OPTION "This is settable from the command line" OFF)
```

For the `BOOL` datatype, there are several different wordings for `ON` and `OFF`

See [cmake-variables](https://cmake.org/cmake/help/latest/manual/cmake-variables.7.html) for a listing of known variables in CMake.



Envrionment variables

You can also `set(ENV{variable_name} value)` and get `$ENV{variable_name}` envrionment variables, though it is generally a very good idea to avoid them.



The Cache

The cache is actually just a text file, `CMakeCache.txt`, that gets created in the build directory when you run CMake. This is how CMake remembers anything you set, so you don’t have to re-list your options every time you rerun CMake.



Properties

The other way CMake stores informations is in properties. This is like a variable, but it is attached to some other item, like a directory or a target. A global property can be a useful uncached global variable.Many target properties are initialized from a matching variable with `CMAKE_` at the front. So setting `CMAKE_CXX_STANDARD`, for example, will mean that all new targets created will have `CXX_STARNARD` set to that when they are created. There are two ways to set properties:

```cmake
set_property(TARGET TargetName
			 PROPERTY CXX_SANDARD 11)
set_target_properties(TargetName PROPERTIES
					  CXX_STANDARD 11)
```

The first form is more general, and can set multiple targets/files/tests at onece, and has useful options. The second is a shortcut for setting several properties on one target. And you can get properties similarly:

```cmake
get_property(ResultVariable TARGET TargetName PROPERTY CXX_STANDARD)
```

See [cmake-properties](https://cmake.org/cmake/help/latest/manual/cmake-properties.7.html) for a listing of all known properties. You can also make your own in some cases.[2](https://cliutils.gitlab.io/modern-cmake/chapters/basics/variables.html#fn_2)



Programming in CMake

Control flow 

```cmake
if (variable)
	# If variable is `ON`, `YES`, `TRUE`, `Y`, or non zero number
else ()
	# If variable is `0`, `OFF`, `NO`, `FALSE`, `N`, `NOTFOUND`, `""`, or ends in `-NOTFOUND`
endif()
	# If variable does not expand to one of the above, CMake wil expand it then try again
```

```cmake
if (${variable})
	# True if variable is not false-like
else ()
	# Note that undefined variables would be `""` thus false
endif()
```

There are a variety of keywords as well, such as:

- Unary: `NOT`, `TARGET`, `EXISTS` (file), `DEFINED`, etc.
- Binary: `STREQUAL`, `AND`, `OR`, `MATCHES` (regular expression), `VERSION_LESS`, `VERSION_LESS_EQUAL` (CMake 3.7+), etc.
- Parentheses can be used to group



generator-expressions

[generator-expressions](https://cmake.org/cmake/help/latest/manual/cmake-generator-expressions.7.html) are really powerful, but a bit odd and specialized. Most CMake commands happen at configure time, include the if statements seen above. But what if you need logic to occur at build time or even install time?Generator expressions were added for this purpose. (They act as if they are evaluated at build/install time, though actually they are evaluated for each build configuration.)They are evaluated in target properties.

The simplest generator expressions are informational expressions, and are of he form `$<KEYWORD>`; they evaluate to a piece of information relevant for the current configuration. The other form is `$<KEYWORD:value>`, where `KEYWORD` is a keyword that controls the evaluation, and value is the item to evaluate (an informational expression keyword is allowed here, too).If KEYWORD is a generator expression or variable that evaluates to 0 or 1, `value` is substituted if ` and not if 0. You can nest generator expressions, and you can use variables to make reading nstd variables bearable.Some expressions allow multiple values, separated by commas.

if you want to put a compile flag only for the DEBUG configuration, for example, you could do:

```cmake
target_compile_options(MyTarget PRIVATE "$<$<CONFIG:Debug>:--my-flag>")
```

> 译者注：这里有点迷惑性，这里其实包含了两种 generator-expression，分别是 configuration-expression 和 conditional-expression，前者使用的形式是 `$<CONFIG:cfgs>`，这里的 cfgs 是一个 List，如果 CONFIG 满足 cfgs 列表中的任何一个值，这个表达式会被评估（evaluate）为 1，否则为 0。后者使用的形式是 `$<condition:true_string>`，如果 condition 值为 1，则表达式被评估为 true_string，否则为空值。因此这里表达的含义是，如果这里是一个 DEBUG 的 configuration，就设置 --my-flag。可参见[官方文档](https://cmake.org/cmake/help/latest/manual/cmake-generator-expressions.7.html#genex:condition)。

This is a newer, better way to add things than using specialized`*_DEBUG` variables, and generalized to all the things generator expressions support.Note that you should never, never, use the configure time value for the current configuration, because multi-configuration generators like IDES do not have a “current” configuration at configure time, only at build time through generator expressions and custon `*_<CONFIG>` variables.

Other common uses for generator expressions:

- Limiting an item to a certain language only, such as CXX, to avoid it mixing with something like CUDA, or wrapping it so that it is different depending on target language.
- Accessing configuration dependent properties, like target file location.
- Giving a different location for build an install directories.

That last one is very common. You’ll see something like this in almost every pakage that supports installing:

```cmake
target_include_directories(
	MyTarget
  PUBLIC 
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>)
    #<INSTALL_INTERFACE:include>
```

> 译者注：表示在目标对于直接 BUILD 使用的目标包含的头文件目录为 `${CMAKE_CURRENT_SOURCE_DIR}/include`，而安装的目标包含的头文件目录为 `include`，是一个相对位置（同时需要 install 对应的头文件才可以）。



Macros and Functions

The only difference between a function and a macro is scope; macros don’t have one.So, if you set a variable in a function and want it to be visible outside, you’ll need `PARENT_SCOPE`. Nesting functions therefore is a bit tricky, since you’ll have to explicitly set the variables you want visible to the outside world to `PARENT_SCOPE` in each function. But, functions don’t “leak” all theirvariables like macros do.

An example of a simple function is as follows：

```cmake
function(SIMPLE REQUIRED_ARG)
	message(STATUS "Simple arguments: ${REQUIRED_ARG}, followed by ${ARGN}")
	set(${REQUIRED_ARG} "From SIMPLE" PARENT_SCOPE)
endfunction()

simple(This Foo Bar)
message("Output: ${This}")
```

The output would be:

```cmake
-- Simple arguments: This, followed by Foo;Bar
Output: From SIMPLE
```

If you want positonal arguments, they are listed explicitly, and all other arguments are colleted in `ARGN` (`ARGV` holds all arguments, even the ones you list). You have to work around the fact that CMake does not have return values by setting variables. In the example above, you can explicitly give a variable name to set.



Arguments

CMake has a named variable system that you’ve already seen in most of the build in CMake functions. You can use it with the [`cmake_parse_arguments`](https://cmake.org/cmake/help/latest/command/cmake_parse_arguments.html) function. If you want to support a version of CMake less than 3.5, you’ll want to also include the [CMakeParseArguments](https://cmake.org/cmake/help/latest/module/CMakeParseArguments.html) module, which is where it used to live before becoming a built in command.

```cmake
function(COMPLEX)
	cmake_parse_arguments(
		COMPLEX_PREFIX
		"SINGLE;ANOTHER"
		"ONE_VALUE;ALSO_ONE_VALUE"
		"MULTI_VALUES"
		${ARGN}
	)
endfunction()

complex(SINGLE ONE_VALUE value MULTI_VALUES some other values)
```

Inside the function after this call, you’ll find

```cmake
COMPLEX_PREFIX_SINGLE = TRUE
COMPLEX_PREFIX_ANOTHER = FALSE
COMPLEX_PREFIX_ONE_VALUE = "value"
COMPLEX_PREFIX_ALSO_ONE_VALUE = <UNDEFINED>
COMPLEX_PREFIX_MULTI_VALUES = "some;other;values"
```

any remaning arguments(therefore optional positional arguments) are in `COMPLEX_PREFIX_UNPARSED_ARGUMENTS`.



### Communicating with your code

**Configure File**

CMake allows you to access CMake variables from your code using `configure_file`. This command copies a file (traditionally ending in `.in`) from one place to another, substituting all CMake variables it finds. If you want to avoid replacing existing `${}` syntax in your input file, use the `@ONLY` keyword. There’s also a `COPY_ONLY` keyword if you are just using this as a replacement for `file(COPY `.

```
Version.h.in
#pargma once

#define MY_VERSION_MAJOR @PROJECT_VERSION_MAJOR@
#define MY_VERSION_MINOR @PROJECT_VERSION_MINOR@
#define MY_VERSION_PATCH @PROJECT_VERSION_PATCH@
#define MY_VERSION_TWEAK @PROJECT_VERSION_TWEAK@
#define MY_VERSION @PROJECT_VERSION@
```

CMake lines:

```
configure_file (
	"${PROJECT_SPIRCE_DIR}/include/My/Version.h.in"
	"${PROJECT_BINARY_DIR}/include/My/Version.h"
)
```

You should include the binary include directory as well when building your project. If you want to put any true/false variables in a header, CMake has C sepcific `#cmakedefine` and `#cmakedefine01` replacements to make apprropriate define lines.

You can also (and often do) use this to produce `.cmake` files, such as the configure files (see [installing](https://cliutils.gitlab.io/modern-cmake/chapters/install/installing.html)).



**Reading files**

The other direction can be done too; you can read in something(like a version) from your source files. If you have a header only library that you’d like to make available with or without CMake, for example, then this would be the best way to handle a version. This would look something like this:

```cmake
# Assuming the canonical version is listed in a single line
# This would be in several parts if picking up from MAJOR, MINOR, etc.
set(VERSION_REGEX "#define MY_VERSION[ \t]+\"(.+)\"")

# Read in the line containing the version
file(STRINGS "${CMAKE_CURRENT_SOURCE_DIR}/include/My/Version.hpp"
	VERSION_STRING REGEX ${VERSION_REGEX})
	
# Pick out just the version
string(REGEX REPLACE ${VERSION_REGEX} "\\1" VERSION_STRING "${VERSION_STRING}")

# Automatically getting PROJECT_VERSION_MAJOR, MY_VERSION_MAJOR, etc.
project(My LANGUAGES CXX VERSION ${VERSION_STRING})
```

Above, `file(STRINGS file_name variable_name REGEX regex)` picks lines that match a regex; and the same regex is used to then pick out the parentheses capture group with the version part. Replace is used with back substitution to output only that one group.



### How to structure your project

First this is what your files should look like when you start if your project is creatively called `project`, with a library called `lib`, and a executable call `app` :

```
- project
  - .gitignore
  - README.md
  - LICENSE.md
  - CMakeLists.txt
  - cmake
    - FindSomeLib.cmake
    - something_else.cmake
  - include
    - project
      - lib.hpp
  - src
    - CMakeLists.txt
    - lib.cpp
  - apps
    - CMakeLists.txt
    - app.cpp
  - tests
    - CMakeLists.txt
    - testlib.cpp
  - docs
    - CMakeLists.txt
  - extern
    - googletest
  - scripts
    - helper.py
```

Notice a few things already apparent; the `CMakeLists.txt` files are split up over all source directories, and are not in the include directories. This is beacause you should be able to copy the contents of the include directory to `/usr/include` or similar directly(except for configuration headers) , and not have any extra files or cause any conflicts.That’s also why there is a directory for your project inside the include directory. Use `add_subdirectory` to add a subdirectory containing a `CMakeLists.txt`.

> 可以注意到一些很明显的问题， `CMakeLists.txt` 文件被分割到除了 `include` 目录外的所有源代码目录下。这是为了能够将 `include` 目录下的所有文件拷贝到 `/usr/include` 目录或其他类似的目录下（除了配置的头文件，这个我将会在另一章讲到），因此为了避免冲突等问题，**其中不能有除了头文件外的其他文件。**这也是为什么在 `include` 目录下有一个名为项目名的目录。顶层 `CMakeLists.txt` 中应使用 `add_subdirectory` 命令来添加一个包含 `CMakeLists.txt` 的子目录。

You often want a `cmake` folder, with all of your heper modules. This is where your `Find*.cmake` files go. An set of some common helpers is at [github.com/CLIUtils/cmake](https://github.com/CLIUtils/cmake). To add this folder to your CMake path:

```cmake
set(CMAKE_MOUDLE_PATH "${PROJECT_SOURCE_DIR}/cmake" ${CMAKE_MOUDLE_PATH})
```

Your `extern` folder should contain git submodules almost exclusively. Thay way, you can control the version of the dependencies explicitly, but still upgrade easily. See the Testing chapter for an example of add a submodule.

You should have something like `/build*` in your `.gitignore`, so that users can make build directories in the source directory and use those to build.A few packages prohibit this, but it’s much better than doing a true out-of-source build and having to type something different for each package you build.

If you want to avoid the build directory being in a valid source directory, add this near the top of your CMakeLists:

```cmake
### Require out-of-source builds
file(TO_CMAKE_PATH "${PROJECT_BINARY_DIR}/CMakeLists.txxt" LOC_PATH)
if (EXISTS "${LOC_PATH}")
	message(FATAL_ERROR "You cannot build in a source directory (or any directory with a CMakeLists.txt file). Please make a bild subdirectory. Feel free to remove CMakeCache.txt and CMakeFiles.")
endif()
```

See the [extended code example here](https://gitlab.com/CLIUtils/modern-cmake/tree/master/examples/extended-project).



### Running other programs

**Running a command at configure time**

Runing a command at configure time is relatively easy. Use [`execute_process`](https://cmake.org/cmake/help/latest/command/execute_process.html) to run a process and access the results. It is gernerally a good idea to avoid hard coding a program path into your CMake; you can use `${CMAKE_COMMAND}` , `find_package(Git)` , or `find_program` to get access to a command to run. Use `RESULT_VARIABLE` to check the return code and `OUTPUT_VARIABLE` to get the output.

Here is an example that updates all git submodules：

```cmake
find_package(Git QUIET)

if(GIT_FOUND AND EXISTS “{PROJECT_SOURCE_DIR}/.git)
	execute_process(COMMAND ${GIT_EXECUTABLE} submodule update --init --recursive
					WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
					RESULT_VARIABLE GIT_SUBMOD_RESULT)
	if(NOT GIT_SUBMOD_RESULT EQUAL "0")
		message(FATAL_ERROR "git submodule update --init --recursive failed with ${GIT_SUBMOD_RESULT}, please checkout submodules")
	endif()
endif()
```

 

**Running a command at build time**

Build time comands are a bit trickier. The main complication comes from the target system; when do you want your comand to run? Does it produce an output that another target needs?With this in mind, here is an example that calls a Python script to generate a header file:

```cmake
find_package(PythonInterp REQUIRED)
add_custom_command(OUTPUT "${CMAKE_CURRENT_BINARY_DIR}/include/Generated.hpp"
				   COMMAND "${PYTHON_EXECUTABLE}" "${CMAKE_CURRENT_SOURCE_DIR}/scripts/GenerateHeader.py" -- argument
				   DEPENDS some_target)
add_custom_target(generate_header ALL DEPENDS "${CMAKE_CURRENT_BINARY_DIR}/include/Generated.hpp")

install(FILES ${CMAKE_CURRENT_BINARY_DIR}/include/Generated.hpp DESTINATION include)
```

Here, the generation happens after `some_target` is complete, and happens when you run make without a target(`ALL`). If you make this a dependency of another target with `add_dependencies`, you could avoid the `ALL` keyword. Or, you could require that a user explicitly builds the `generate_header` target when making.



Included common utilities

A useful tool in writing CMake builds that work cross-platform is `cmake -E <mode>` (seen in CMake files as `${CMAKE_COMMAND} -E`) . This mode allows CMake to do a variety of things without calling system tools explicitly, like `copy`, `make_directory` and `remove`. It is mostly used for the build time commands. Note that the very useful `create_symlink` mode used to be Unix only, but was added for Windows in CMake 3.13. [See the docs](https://cmake.org/cmake/help/latest/manual/cmake.1.html#command-line-tool-mode).



### A simple example

For this program, we have one library (MyLibExample) with a header file and a source file, and one application, MyExample, with one source file.

```cmake
# Almost all CMake files should start with this.
# You should always specify a range with the newest
# and oldest tested versions of CMake. This will ensure
# you pick up the best policies .
cmake_minimum_required(VERSION 3.1...3.26)

# This is your project statement. You should always list languages;
# Listing the version is nice here since it sets lots of useful variables
projects(
	ModernCMakeExamle
	VERSION 1.0
	LANGUAGES CXX)

# If you set any CMAKE_ variables, that can go here.
# (But usually don't do this, except maybe for c++ standard)

# Find packages go here.

# You should usually split this into folders, but this is a simple example

# This is a "default" library, and will match the *** variable setting.
# Other common choices are STATI, SHARED, and MODULE
# Including header files here helps IDEs but is not required.
# Output libname matches target name, with the usual extensions on your system
add_library(MyLibExample simple_lib.cpp simple_lib.hpp)

# Link each target with other targets or add options, etc.
add_executable(MyExample simple_example.cpp)

# Make sure you link your targets with this command. It can also link libraries and 
# even flags, so linking a target that does not exist will not give a configure-time error.
target_link_libraries(MyExample PRIVATE MyLibExample)
```

The complete example is available in [examples folder](https://gitlab.com/CLIUtils/modern-cmake/tree/master/examples/simple-project).

A larger, multi-file example is [also available](https://gitlab.com/CLIUtils/modern-cmake/tree/master/examples/extended-project).



## Adding features

Default build type

CMake normally does a “non-release, non debug” emptydd build type; if you perfer to set the default build type yourself , you can follow this for the default build type modified from the [Kitware blog](https://blog.kitware.com/cmake-and-the-default-build-type/):

```cmake
set(default_build_type "Release")
if(NOT CMAKE_BUILD_TYPE AND NOT CMAEK_CONFIGURATION_TYPES)
	message(STATUS "Setting build type to '${default_build_type}' as none was specified.")
	set(CMAKE_BUILD_TYPE "${default_build_type}" CACHE
		STRING "Choose the type of build" FROCE)
	# Set the possible values of build type for cmake-gui
	set_property(CACHE CMAKE_BUILD_TYPE PROPERTY STRINGS
		"Debug" "Release" "MinSizeRel" "RelWithDebInfo")
endif(
```



CMake 3.8+: Meta compiler features

Assuming you have a target named `myTarget`, it looks like this:

```cmake
target_compile_features(myTarget PUBLIC cxx_std_11)
set_target_properties(myTarget PROPERTIES CXX_EXTENSIONS OFF)
```

For the first line, we get to pick between `cxx_std_11`, `cxx_std_14`, and `cxx_std_17`. The secone line is optional, but will avoid extensions being added; without it you’d get things like `-std=g++11` replacing `-std=c++11`. The first line even works on `INTERFACE` targets; only actual compiled targets can use the second line.

If a target further down the dependenvy chain specifies a higher C++ level, this interacts nicely. It’s really just a more advanced version of the following method, so it interacts nicely with that, too.



CMake 3.1+: Compiler features

You can ask for specific compiler features to be availabele. This was more granular than asking for a C++ version, though it’s a bit tricky to pick out just the features a package is using unless you wrote the package and have a good memory. Since this ultimately checks against a list of options CMake knows your compiler supports and picks the highest flag indicated in that list, you don’t have to specify all the options you need, just he rarest ones. The syntax is identical to teh section above, you just have a list of options to pick instead of `cxx_std_*` options. See the [whole list here](https://cmake.org/cmake/help/latest/prop_gbl/CMAKE_CXX_KNOWN_FEATURES.html).



CMAKE 3.1+: Global and property settings

There is another way that C++ standards are supported; a specific set of three properties(both global and target level). The global properties are:

```cmake
set(CMAKE_CXX_STANDARD 11 CACHE STRING "The C++ standard to use")
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
```

The first line sets a C++ standard level, and the second tells CMake to use it, and the final line is optional and ensures `-std=c++11` vs. something like `-std=g++11`. This method isn’t bad for a final package, but shouldn’t be used by a library. You should always set this as a cached variable, **so you can override it to try a new version easily**(or if this gets used as a library, this is the only way to override it - but again, don’t use this for libraries). You can also set these values on a target:

```cmake
set_target_properties(myTarget PROPERTIES
	CXX_STANDARD 11
	CXX_STANDARD_REQUIRED YES
	CXX_EXTENSIONS NO
)
```

which is better, but still doesn’t have the sort of explicit control that compiler features have for populating `PRIVATE` and `INTERFACE` properties, **so it really is only useful on final targets.**

You can find more information about the final two methods on [Craig Scott's useful blog post](https://crascit.com/2015/03/28/enabling-cxx11-in-cmake/).

> **Don't set manual flags yourself.** You'll then become responsible for mainting correct flags for every release of every compiler, error messages for unsupported compilers won't be useful, and some IDEs might not pick up the manual flags.



### Small but common needs

**Adding Features**

There are lots of compiler and linker settings. When you need to add something special, **you could check first to see if CMake supports it;** if it does, you can avoid explicitly tying yourself to a compiler version.And better yet, you explain what you mean in your CMakeLists, rather thann spewing flags.



**Posotion independent code**

[This](https://cmake.org/cmake/help/latest/variable/CMAKE_POSITION_INDEPENDENT_CODE.html) is best known as the `-fPIC` flag. Much of the time, you don’t need to do anything. CMake will include the flag for `SHARED` or `MODULE` libraries. If you do explicitly need it:

```cmake
set(CMAKE_POSITION_INDEPENDENT_CODE ON)
```

will do it globally, or:

```cmake
set_target_properties(lib1 PROPERTIES POSITION_INDEPENDENT_CODE ON)
```

to explicitly turn it `ON` (or `OFF`) for a target.



**Little libraries**

If you need to link to the `dl` library, with `-ldl` on Linux, just use the built-in CMake variable `${CMAKE_DL_LIBS}` in a `target_link_libraries` command. No module or `find_package` needed. (This adds whatever is needed to get `dlopen` and `dlclose`)

Unfortunately, the math library is not so lucky. If you need to explicitly link to it, you can always do `target_link_libraries(MyTarget PUBLIC m)` , but it might be better to use CMake’s generic `find_library` :

```cmake
find_library(MATH_LIBRARY m)
if(MATH_LIBRARY)
	target_link_libraries(MyTarget PUBLIC ${MATH_LIBRARY})
endif()
```

You can pretty easily find `Find*.cmake`'s for this and other libraries that you need with a quick search;most major packages have a helper library of CMake modules. 



Interprocedural optimization

[`INTERPROCEDURAL*OPTIMIZATION`](https://cmake.org/cmake/help/latest/prop_tgt/INTERPROCEDURAL*OPTIMIZATION.html),best known as _link time optimization* and the `-flto` flag, is available on very recent versions of CMake.You can turn this on with  [`CMAKE_INTERPROCEDURAL_OPTIMIZATION`](https://cmake.org/cmake/help/latest//CMAKE_INTERPROCEDURAL_OPTIMIZATION.html) or the [`INTERPROCEDURAL_OPTIMIZATION`](https://cmake.org/cmake/help/latest/prop_tgt/INTERPROCEDURAL_OPTIMIZATION.html) property on targets. If you set `cmake_minimum_required(VERSION 3.9)` or better (see [CMP0069](https://cmake.org/cmake/help/latest/policy/CMP0069.html)),setting this to `ON` on a target is an error if the compiler doesn’t support it. You can use check_ipo_supported(), from the built-in [CheckIPOSupported](https://cmake.org/cmake/help/latest/module/CheckIPOSupported.html) module, to see if support is available before hand. An example of 3.0 style usage:

```cmake
include(CheckIPOSupported)
check_ipo_supported(RESULT result)
if(result)
	set_target_properties(foo PROPERTIES INTERPROCEDUAL_OPTIMIZATION TRUE)
endif()
```



### Utilities

CCache and Utilities

Over the versions, common utilities that help you write good code have had support added to CMake.This is usually in the form of a property and matching `CMAKE_*` initialization variable. The feature is not meant to be tied to one special program, but rather any program that is somewhat similar in behavior.

All of this take `;` separated values(a standard list in CMake) that describe the program and options that you should run on the source file of this target.



CCache

Set the `CMAKE_<LANG>_CMPILER_LAUNCHAER` variable or the `<LANG>_COMPILER_LAUNCHER` property on a target to use something like CCache to “wrap” the compilation of the target. Support for CCache has been expanding in the latest versions of CMake. In pratice, this tends to look like this:

```cmake
find_program(CCACHE_PROGRAM ccache)
if(CCACHE_PROGRAM)
	set(CMAKE_CXX_COMPILER_LAUNCHER "${CCACHE_PROGRAM}")
	set(CMAKE_CUDA_COMPILER_LAUNCHER "${CCACHE_PROGRAM}")
endif()
```



