# rci - Continuous Integration Environment

[![Build Status](https://travis-ci.org/klingtnet/rci.svg?branch=master)](https://travis-ci.org/klingtnet/rci) [![Circle CI](https://circleci.com/gh/klingtnet/rci.svg?style=svg)](https://circleci.com/gh/klingtnet/rci)

`rci` is wrapper for environment variables of some common continiuous integration services.
At the moment [travis](https://travis-ci.org/) and [circle-ci](https://circleci.com/) is supported.
A possible use case for this library is to check if your tests are running in an contniuous
service.
**Don't** use this to skip all of your tests and pretend everything works fine!
If you are testing for example audio or graphics output
that is not available in certain CI environments
then you can use this library to skip those tests.
