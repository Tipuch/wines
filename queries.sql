SELECT saq.name, saq.country, saq.region, saq.designation_of_origin, saq.producer, saq.color, saq.price, reco.rating FROM saq_wines as saq
JOIN wine_recommendations as reco ON 
(reco.country = '' OR saq.country ILIKE reco.country) AND
(reco.region = '' OR saq.region ILIKE reco.region) AND
(reco.designation_of_origin = '' OR saq.designation_of_origin ILIKE reco.designation_of_origin) AND
(reco.producer = '' OR saq.producer ILIKE reco.producer) AND
(saq.color = reco.color) AND
(reco.grape_variety = '' OR reco.grape_variety ILIKE ANY(saq.grape_varieties))
ORDER BY ratio;