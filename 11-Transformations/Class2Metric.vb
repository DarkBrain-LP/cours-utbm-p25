module Class2Metric;
create OUT aMetric : MMMetrics from IN aClass: MMClass;

helper context MMClass!Class
	def: getNbAttribute() : Integer =
		if not self.attributes.oclIsUndefined() then
			true
		else
			if not self.familyDaughter.oclIsUndefined() then
				true
			else
				false
		endif


rule Class2nbAttributeMetric{
	from 
		c: MMClass!Class
	to
		m: MMMetrics!IntMetric(
			' label <- 'attributes number'
			label <- 'Class' + c.name
			context <- c.name
			' data <- c.getNbAttribute()
			data <- c.references->size()
		)
}