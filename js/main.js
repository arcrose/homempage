(function() {
  const codeSegmentTop = [3, 5]            // Start drawing code from x=3, y=5
  const codeSegmentIdentLineDelta = [6, 3] // Add x+5, y+3 to indent the next line of code

  const lineCharLimits = [
    'AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA'.length,
    'AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA'.length,
    'AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA'.length,
    'AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA'.length,
    'AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA'.length,
    'AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA'.length,
    'AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA'.length,
    'AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA'.length,
    'AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA'.length,
    'AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA'.length,
    'AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA'.length,
    'AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA'.length,
    'AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA'.length,
    'AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA'.length,
    'AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA'.length,
    'AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA'.length,
    'AAAAAAAAAAAAAAAAAAAAAAAAAAAAA'.length,
    'AAAAAAAAAAAAAAAAAAAAAAAAAAA'.length,
    'AAAAAAAAAAAAAAAAAAAAAAAAA'.length,
    'AAAAAAAAAAAAAAAAAAAAAAA'.length,
    'AAAAAAAAAAAAAAAAAAAAA'.length,
    'AAAAAAAAAAAAAAAAAA'.length,
    'AAAAAAAAAAAAAAAA'.length,
    'AAAAAAAAAAAAAA'.length,
    'AAAAAAAAAAAA'.length,
    'AAAAAAAAAA'.length,
    'AAAAAAAA'.length,
    'AAAAAA'.length,
  ]


  const text = (x, y, classes, text) => {
    let newNode = document.createElementNS('http://www.w3.org/2000/svg', 'text');
    newNode.setAttribute('x', `${x}`)
    newNode.setAttribute('y', `${y}`)
    newNode.textContent = text

    for (const className of classes) {
      newNode.classList.add(className)
    }

    return newNode
  }

  const code = (line, indent, code) => {
    const startX = indent * codeSegmentIdentLineDelta[0] + codeSegmentTop[0]
    const startY = line * codeSegmentIdentLineDelta[1] + codeSegmentTop[1]
    const charLimit = lineCharLimits[line] - 4 * indent

    return text(startX, startY, ['code', 'white'], code.slice(0, charLimit))
  }

  const codeSegment = document.getElementById('diagonal-code-segment')
  const writingSegment = document.getElementById('diagonal-writing-segment')

  codeSegment.appendChild(code(0, 0, 'def factorial(n, acc=1):'))
  codeSegment.appendChild(code(1, 1, 'if n <= 1:'))
  codeSegment.appendChild(code(2, 2, 'return acc'))
  codeSegment.appendChild(code(3, 1, 'print("This is a really stupid long line of text frankly you should just pretend"'))
  codeSegment.appendChild(code(4, 1, 'return factorial(n - 1, acc * n)'))

  writingSegment.appendChild(text(10, 95, ['word', 'black'], 'Test'))
})()
