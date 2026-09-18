#!/usr/bin/env node
'use strict';

process.env.FASTCTX_NPM_PACKAGE = '@pennixrv/codex-fastctx';
process.env.FASTCTX_NPM_LAUNCHER = __filename;
require('@pennixrv/fastctx/launcher.js');
