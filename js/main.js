(function() {
  const codeSegment = document.getElementById('diagonal-code-segment')
  const writingSegment = document.getElementById('diagonal-writing-segment')

  const segmentMaxLines = 35
  const codeSegmentTop = [3, 5]            // Start drawing code from x=3, y=5
  const segmentIdentLineDelta = [6, 3] // Add x+6, y+3 to indent the next line of code

  // Produce an HTML <text> element positioned at the given coordinates.
  const text = (x, y, className, text) => {
    let newNode = document.createElementNS('http://www.w3.org/2000/svg', 'text');
    newNode.setAttribute('x', `${x}`)
    newNode.setAttribute('y', `${y}`)
    newNode.textContent = text

    newNode.classList.add(className)

    return newNode
  }

  // Produce an HTML <text> element vertically aligned and trimmed
  // to be displayed in the code segment.
  const code = (line, indent, code) => {
    const startX = indent * segmentIdentLineDelta[0] + codeSegmentTop[0]
    const startY = line * segmentIdentLineDelta[1] + codeSegmentTop[1]

    return text(startX, startY, 'code', code)
  }

  const writing = (lineNumber, lineText) => {
    const startX = 5 + 0.5 * lineNumber * segmentIdentLineDelta[0]
    const startY = 98 - lineNumber * segmentIdentLineDelta[1]
  
    return text(startX, startY, 'writing', lineText)
  }

  // Construct a representation of a state machine that can be operated on
  // to animate text scrolling through the code segment.
  const codeAnimator = (language, source) => {
    const start = Math.min(segmentMaxLines, source.linesOfCode.length)
    return {
      _startLine: start,
      _linesBuffer: source.linesOfCode.slice(0, start),
      _language: language,
      _source: source,
    }
  }

  const writingAnimator = sample => {
    const start = Math.min(segmentMaxLines, sample.lines.length)
    let buffer = sample.lines.slice(0, start)
    for (let i = 0; i < segmentMaxLines - buffer.length; i++) {
      buffer.push({ line: '' })
    }
    return {
      _startLine: 0,
      _linesBuffer: buffer,
      _sample: sample,
    }
  }

  const _codeAnimatorStep = animator => {
    const codeToText = (state, line) => {
      state.nodes.push(code(state.lineNumber, line.indent, line.code))
      return {
        nodes: state.nodes,
        lineNumber: state.lineNumber + 1,
      }
    }
    
    let init = {
      nodes: [],
      lineNumber: 0,
    }

    const { nodes } = FN.reduce(codeToText, animator._linesBuffer, init)
    
    animator._linesBuffer = animator._linesBuffer.slice(1)
    animator._linesBuffer.push(animator._source.linesOfCode[animator._startLine])
    animator._startLine++

    return {
      animator,
      nodes,
    }
  }

  const _writingAnimatorStep = animator => {
    const writingToText = (state, line) => {
      if (typeof line !== 'undefined' && typeof line.text !== 'undefined') {
        state.nodes.push(writing(state.lineNumber, line.text))
      }
      return {
        nodes: state.nodes,
        lineNumber: state.lineNumber + 1,
      }
    }

    let init = {
      nodes: [],
      lineNumber: 0,
    }

    const { nodes } = FN.reduce(writingToText, animator._linesBuffer, init)

    animator._linesBuffer = animator._linesBuffer.slice(1)
    animator._linesBuffer.push(animator._sample.lines[animator._startLine])
    animator._startLine++

    return {
      animator,
      nodes,
    }
  }

  const schedule = (initState, loopPause, operations) => ({
    _state: initState,
    _loop: loopPause,
    _ops: operations,
    _index: 0,
  })

  const _run = schedule => {
    setTimeout(() => {
      const op = schedule._ops[schedule._index]
      schedule._state = op(schedule._state)
      schedule._index = (schedule._index + 1) % schedule._ops.length
      _run(schedule)
    }, schedule._loop)
  }

  const drawCode = ({ animator, nodes }) => {
    const { animator: a, nodes: n } = _codeAnimatorStep(animator)

    let newNodes = []
    for (let i = 0; i < nodes.length; i++) {
      nodes[i].remove();

      const node = n[i]
      newNodes.push(node)
      codeSegment.appendChild(node)
    }
    if (n.length > nodes.length) {
      const node = n[n.length - 1]
      newNodes.push(node)
      codeSegment.appendChild(node)
    }

    return {
      animator: a,
      nodes: newNodes,
    }
  }

  const drawWriting = ({ animator, nodes }) => {
    const { animator: a, nodes: n } = _writingAnimatorStep(animator)

    let newNodes = []
    const minIndex = Math.min(n.length, nodes.length)
    for (let i = minIndex - 1; i >= 0; i--) {
      nodes[i].remove()
      newNodes.push(n[i])
    }
    if (n.length > nodes.length) {
      newNodes.push(n[n.length - 1])
    } else if (nodes.length > n.length) {
      for (let i = n.length; i < nodes.length; i++) {
        nodes[i].remove()
      }
    }
    for (const node of newNodes) {
      writingSegment.appendChild(node)
    }

    return {
      animator: a,
      nodes: newNodes,
    }
  }

  const drawAllCode = animator => {
    const { animator: a, nodes } = _codeAnimatorStep(animator)

    for (const node of nodes) {
      codeSegment.appendChild(node)
    }

    return {
      animator: a,
      nodes,
    }
  }

  const drawAllWriting = animator => {
    const { animator: a, nodes } = _writingAnimatorStep(animator)

    for (const node of nodes) {
      writingSegment.appendChild(node)
    }

    return {
      animator: a,
      nodes,
    }
  }

  const prepareCodeAnimator = ({ animator, nodes }) => {
    if (animator._startLine === animator._source.linesOfCode.length - segmentMaxLines) {
      animator = resetCodeAnimator(animator)
    }
    return {
      animator,
      nodes,
    }
  }

  const prepareWritingAnimator = ({ animator, nodes }) => {
    if (animator._startLine >= animator._sample.lines.length - segmentMaxLines) {
      animator = resetWritingAnimator(animator)
    }
    return {
      animator,
      nodes,
    }
  }
  
  const resetCodeAnimator = animator => {
    const snippet = pickRandom(CODE_SNIPPETS)
    animator._source = pickRandom(snippet.sourceFiles)
    animator._startLine = 0
    return animator
  }

  const resetWritingAnimator = animator => {
    animator._sample = pickRandom(WRITING_SAMPLES)
    animator._startLine = 0
    return animator
  }

  const pickRandom = list => {
    const index = Math.floor(Math.random() * list.length)
    return list[index]
  }

  const main = () => {
    const snippet = pickRandom(CODE_SNIPPETS)
    const source = pickRandom(snippet.sourceFiles)

    let codeA = codeAnimator(snippet.languageName, source)
    let { animator: newCodeA, nodes: codeNodes } = drawAllCode(codeA)

    let sched = schedule({ animator: newCodeA, nodes: codeNodes }, 200, [ drawCode, prepareCodeAnimator ])

    _run(sched)


    const sample = pickRandom(WRITING_SAMPLES)

    let writingA = writingAnimator(sample)
    let { animator: newWritingA, nodes: writingNodes } = drawAllWriting(writingA)

    sched = schedule({ animator: newWritingA, nodes: writingNodes }, 200, [ drawWriting, prepareWritingAnimator ])

    _run(sched)
  }

  main()
})()
