project = 'notes'
templates_path = ['_templates']
exclude_patterns = ['_build', 'Thumbs.db', '.DS_Store']
html_static_path = ['_static']

# Same as our License header.
copyright = 'Kazuyoshi Kato'
author = 'Kazuyoshi Kato'

# MyST (Markedly Structured Text) is a superset of CommonMark.
# https://myst-parser.readthedocs.io/en/latest/
extensions = [ 'myst_parser' ]

html_theme = 'furo'
