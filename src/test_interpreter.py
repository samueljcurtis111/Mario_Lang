import re
import sys
import os


COUNT_TO_BF = {
    1: "+",
    2: "-",
    3: ">",
    4: "<",
    5: ".",
    6: ",",
    7: "[",
    8: "]",
}


def tokenize(code):
    blocks = re.split(r"\s{2,}|\n+", code.strip())

    tokens = []
    for block in blocks:
        mario_count = len(re.findall(r"\bmario\b", block, re.IGNORECASE))
        if mario_count in COUNT_TO_BF:
            tokens.append(COUNT_TO_BF[mario_count])

    return tokens


def build_jump_map(tokens):
    stack = []
    jump_map = {}
    for ip, token in enumerate(tokens):
        if token == "[":
            stack.append(ip)
        elif token == "]":
            if not stack:
                raise SyntaxError("Unmatched ']' in code")
            start = stack.pop()
            jump_map[start] = ip
            jump_map[ip] = start
    if stack:
        raise SyntaxError("Unmatched '[' in code")
    return jump_map


def evaluate(code):
    tokens = tokenize(code)
    jump_map = build_jump_map(tokens)

    memory = [0] * 30000
    ptr = 0
    ip = 0
    code_len = len(tokens)

    while ip < code_len:
        cmd = tokens[ip]

        if cmd == "+":
            memory[ptr] = (memory[ptr] + 1) % 256
        elif cmd == "-":
            memory[ptr] = (memory[ptr] - 1) % 256
        elif cmd == ">":
            ptr = (ptr + 1) % 30000
        elif cmd == "<":
            ptr = (ptr - 1) % 30000
        elif cmd == ".":
            sys.stdout.write(chr(memory[ptr]))
            sys.stdout.flush()
        elif cmd == ",":
            char = sys.stdin.read(1)
            memory[ptr] = ord(char) if char else 0
        elif cmd == "[":
            if memory[ptr] == 0:
                ip = jump_map[ip]
        elif cmd == "]":
            if memory[ptr] != 0:
                ip = jump_map[ip]

        ip += 1


if __name__ == "__main__":
    # Rules:
    # Double spaces (  ) or newlines separate instructions.
    # 1 mario = +
    # 2 marios = -
    # 3 marios = >
    # 4 marios = <
    # 5 marios = .
    # 7 marios = [
    # 8 marios = ]

    with open("src/code.mario", "r", encoding="utf-8") as file:
        file_contents = file.read()



    mario_program = file_contents

    evaluate(mario_program)