Context: Reunion
inv FinApresDebut: self.Fin > self.Debut

Context: Reunion
inv MustHaveModerator: self.Moderateur.notNil()

Context: Reunion::isConfirmed: Boolean
init false

Context: Reunion::duree : Integer
derive self.fin - self.debut