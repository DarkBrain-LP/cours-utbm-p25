Context: Vehicule
inv Fonctionne: enMarche implies (capacite > 0 and aMoteur.notNil() and 
                                    aMoteur.isDemarre and conducteur.notNil())

Context: Vehicule
inv self.capacite < self.capaciteMax

Context: Modeur
inv self.isDemarre implies (self.MoteurDe.notNil() and 
                            self.MoteurDe.enMarche)

Context: Personne
inve: self.conduit.notNil() implies self.age > 18