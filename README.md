<div align="center">
<h1>rqalam</h1>
<p>
  Bytecode compiler for the <a href="https://github.com/ammar-ahmed22/qalam">Qalam</a> programming langauge.
</p>
</div>

## Table of Contents

## Introduction
After completing my interpreted version of [Qalam](https://github.com/ammar-ahmed22/qalam), I'm following the next part of [Robert Nystrom's Tutorial](https://craftinginterpreters.com/), to create a (hopefully) much faster, compiled version, of `qalam`. 

The initial version of `qalam` creates an AST (abstract syntax tree) and traverses the tree, executing all the statements. This is extremely slow, especially when it gets to things such as recursive calls. For this reason, Robert implements a compiled version which compiles to bytecode, making the execution significantly faster.

As with the previous implementation, I didn't want to simply copy-paste a tutorial so I'm going to be using Rust once again instead of C++. 

The details of Qalam's syntax and langauge features can be found [here](https://github.com/ammar-ahmed22/qalam).