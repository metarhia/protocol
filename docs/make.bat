@ECHO OFF

pushd %~dp0

REM Command file for Sphinx documentation

if "%SPHINXBUILD%" == "" (
	set SPHINXBUILD=python -msphinx
)
if "%SPHINXAUTOBUILD%" == "" (
	set SPHINXAUTOBUILD=python -msphinx_autobuild
)
if "%SPHINXPORT%" == "" (
	set SPHINXPORT=8000
)
set SOURCEDIR=.
set BUILDDIR=_build
set SPHINXPROJ=MetarhiaProtocol

if "%1" == "" goto help
if "%1" == "livehtml" goto livehtml

%SPHINXBUILD% >NUL 2>NUL
if errorlevel 9009 (
	echo.
	echo.The Sphinx module was not found. Did you forget to run
	echo."npm run docs-install-deps" or activate a virtual environment?
	exit /b 1
)

%SPHINXBUILD% -M %1 %SOURCEDIR% %BUILDDIR% %SPHINXOPTS%
goto end

:livehtml
%SPHINXAUTOBUILD% >NUL 2>NUL
if errorlevel 9009 (
	echo.
	echo.The sphinx-autobuild module was not found. Did you forget to run
	echo."npm run docs-install-deps" or activate a virtual environment?
	exit /b 1
)

%SPHINXAUTOBUILD% %SOURCEDIR% %BUILDDIR%\html -p %SPHINXPORT% %SPHINXOPTS%
goto end

:help
%SPHINXBUILD% -M help %SOURCEDIR% %BUILDDIR% %SPHINXOPTS%

:end
popd
