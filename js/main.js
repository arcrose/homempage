(function() {
  const codeSegmentTop = [3, 5]            // Start drawing code from x=3, y=5
  const codeSegmentIdentLineDelta = [6, 3] // Add x+5, y+3 to indent the next line of code


  const text = (x, y, className, text) => {
    let newNode = document.createElementNS('http://www.w3.org/2000/svg', 'text');
    newNode.setAttribute('x', `${x}`)
    newNode.setAttribute('y', `${y}`)
    newNode.textContent = text

    newNode.classList.add(className)

    return newNode
  }

  const code = (line, indent, code) => {
    const startX = indent * codeSegmentIdentLineDelta[0] + codeSegmentTop[0]
    const startY = line * codeSegmentIdentLineDelta[1] + codeSegmentTop[1]

    const charLimit = 64 - 2.25 * line - 5 * indent

    return text(startX, startY, 'code', code.slice(0, charLimit))
  }

  const codeSegment = document.getElementById('diagonal-code-segment')
  const writingSegment = document.getElementById('diagonal-writing-segment')

  codeSegment.appendChild(code(0, 0, 'def factorial(n, acc=1):'))
  codeSegment.appendChild(code(1, 1, 'if n <= 1:'))
  codeSegment.appendChild(code(2, 2, 'return acc'))
  codeSegment.appendChild(code(3, 1, 'print("This is a really stupid long line of text frankly you should just pretend"'))
  codeSegment.appendChild(code(4, 1, 'print("This is a really stupid long line of text frankly you should just pretend"'))
  codeSegment.appendChild(code(5, 2, 'print("This is a really stupid long line of text frankly you should just pretend"'))

  writingSegment.appendChild(text(10, 95, 'word', 'Test'))
})()
