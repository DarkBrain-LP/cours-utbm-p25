Context Personne
inv Majorite: self.isMajeur implies self.age >= 18
inv Maternite: self.Mere <> self
// inv MariageFemme: self.mari.notNil() implies (self.isMajeur and self.epouse.notNil() 
//                                             and self.epouse.isMajeur)
// inv MariageHomme: self.epouse.notNil() implies (self.isMajeur and self.mari.notNil() 
//                                             and self.mari.isMajeur)
// les deux précédents sont fuax, voici la correction:
inv Mariage: self.epouse.notNil() implies self.epouse.age > 18
                and self.mari.notNil() implies self.mari.age > 18

Context Companie
inv Director: self.director.notNil() and self.director.isChomeur = false and self.director.age > 40

Context Personne::revenu_net:Integer
derive if self.isChomeur <> false
            self.revenu_brut*3/4
        else
            0

Context: Personne
inv: self.companie.Nil() and self.isMajeur <> false implies self.isChomeur
inv: self.isChomeur implies self.revenu_net < 500
inv: self.Pere.notNil() and self.Mere.notNil() 
        and self.Pere <> self and self.Mere <> self 
        and self.Mere<>self.Pere
 
def: ArgentPoche: Integer