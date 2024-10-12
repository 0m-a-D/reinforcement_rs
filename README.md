Using "elegibility traces" -> E(s,a) for efficient assigning of rewards for (state, action) pairs.
more specifically: dutch elegibility traces. 

ƛ: trace decay parameter::: set to 0.1
Ɣ: have to fine-tune as training proceeds Ɣ ∈ (0,0.3)

Learning policy: Q-learning instead of SARSA -> q(st,at) = q(st,at) + ⍺(Rt+1 + Ɣmax(q(st,a')) - q(st,at))

-> gradually decrease temperature β in softmax to slowly increase exploitation.
