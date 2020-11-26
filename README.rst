.. image:: https://raw.githubusercontent.com/amoffat/supertag/master/logo/logo.gif
    :target: https://amoffat.github.com/supertag
    :alt: Logo

|

.. image:: https://img.shields.io/travis/amoffat/sh/master.svg?style=flat-square
    :target: https://travis-ci.org/amoffat/sh
    :alt: Build Status
.. image:: https://img.shields.io/coveralls/amoffat/sh.svg?style=flat-square
    :target: https://coveralls.io/r/amoffat/sh?branch=master
    :alt: Coverage Status

|

Supertag is a tag-based filesystem, written in Rust, for Linux and MacOS. It provides a tag-based view of your files by
removing the hierarchy constraints typically imposed on files and folders.

.. image:: https://raw.githubusercontent.com/amoffat/supertag/master/images/intersection-opt.gif
    :alt: Intersection

`Full docs <https://amoffat.github.io/supertag/>`_

Installation
============

Linux
-----

.. code-block:: bash

    sudo apt-add-repository -u ppa:amoffat/supertag
    sudo apt install supertag

Mac
---

.. code-block:: bash

    brew install supertag