# Minimal makefile for Sphinx documentation
#

# You can set these variables from the command line.
SPHINXOPTS      ?=
SPHINXBUILD     ?= python -msphinx
SPHINXAUTOBUILD ?= python -msphinx_autobuild
SPHINXPORT      ?= 8000
SPHINXPROJ      = MetarhiaProtocol
SOURCEDIR       = .
BUILDDIR        = _build

# Put it first so that "make" without argument is like "make help".
help:
	@$(SPHINXBUILD) -M help "$(SOURCEDIR)" "$(BUILDDIR)" $(SPHINXOPTS) $(O)

livehtml:
	$(SPHINXAUTOBUILD) "$(SOURCEDIR)" "$(BUILDDIR)/html" \
		-p $(SPHINXPORT) $(SPHINXOPTS) $(O)

.PHONY: help livehtml Makefile

# Catch-all target: route all unknown targets to Sphinx using the new
# "make mode" option.  $(O) is meant as a shortcut for $(SPHINXOPTS).
%: Makefile
	@$(SPHINXBUILD) -M $@ "$(SOURCEDIR)" "$(BUILDDIR)" $(SPHINXOPTS) $(O)
