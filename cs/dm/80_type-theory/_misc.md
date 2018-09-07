

Lemma [Exchange]: if
$$Γ_{_1}, x_1:T_{_1}, x_2:T_{_2}, Γ_{_2} \vdash t:T$$ then 
$$Γ_{_1}, x_2:T_{_2}, x_1:T_{_1}, Γ_{_2} \vdash t:T$$

Lemma [Weakening]: if
$$Γ_{_1},\quad \quad \quad \ Γ_2 \vdash t:T$$ then 
$$Γ_{_1}, x_1:T_{_1}, Γ_{_2} \vdash t:T$$

Lemma [Contraction]: if
$$Γ_{_1}, x_2 : T_{_1}, x_3 :T_{_1}, Γ_{_2} \vdash t:T_{_2}$$ then 
$$Γ_{_1}, \quad \ x_1:T_{_1},\quad \quad Γ_{_2} \vdash [x_2,x_1][x_3,x_1]t:T_{_2}$$

