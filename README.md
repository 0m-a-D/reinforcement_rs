Using **elegibility traces** -> E(s,a) for efficient assigning of rewards for (state, action) pairs.
more specifically: dutch elegibility traces. 

-- ƛ: trace decay parameter::: set to 0.1

-- ⍺: Learning parameter...⍺ ∈ (0,1]

-- Ɣ: have to fine-tune as training proceeds Ɣ ∈ (0,0.3)

-- R ∈ [0,1]

-- gradually decrease temperature β in softmax to slowly increase exploitation.

Learning policy: [**Q-learning**](https://en.wikipedia.org/wiki/Q-learning) instead of [**SARSA**](https://en.wikipedia.org/wiki/State%E2%80%93action%E2%80%93reward%E2%80%93state%E2%80%93action) -> q(st,at) = q(st,at) + ⍺(Rt+1 + Ɣmax(q(st,a')) - q(st,at))
