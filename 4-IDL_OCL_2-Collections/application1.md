inv: self.isConfirmed implies (
    self.participants -> select(p| p.gender=Gender::male)->size() >= self.participants->size()*0.4
    and self.participants -> select(p| p.gender=Gender::female)->size() >= self.participants->size()*0.4
)

-- Pour confirmer une r´eunion il faut que la dur´ee soit non nulle et que le nombre de participants soit sup´erieur `a 3
context: Meeting::confirm()
pre: self.duree.IsNotNull()
    and self.participants->size() > 3

-- Une personne ne peut pas participer `a deux r´eunions la meme journ´ee
context: Teammember
inv: not(self.reunions -> exists(r1, r2| r1<>r2 and r1.day=r2.day))
or
inv: self.reunions->forAll(r1,r2 | r1<>r2 implies r1.day <> r2.day)