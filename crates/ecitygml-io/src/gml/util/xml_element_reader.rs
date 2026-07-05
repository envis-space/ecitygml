use crate::Error;
use crate::gml::util::xml_element;
use crate::gml::util::xml_element::XmlElement;
use quick_xml::Reader;
use quick_xml::events::Event;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Range;
use tracing::debug;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlElementSpans {
    spans: HashMap<XmlElement, Vec<Range<usize>>>,
}

impl XmlElementSpans {
    pub fn new(spans: HashMap<XmlElement, Vec<Range<usize>>>) -> Self {
        Self { spans }
    }

    pub fn get(&self, element: XmlElement) -> &[Range<usize>] {
        self.spans
            .get(&element)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn first(&self, element: XmlElement) -> Option<&Range<usize>> {
        let spans = self.spans.get(&element)?;
        debug_assert!(
            spans.len() <= 1,
            "expected at most one span for {:?}",
            element
        );
        spans.first()
    }
}

pub fn collect_child<T>(
    xml_document: &[u8],
    spans: &XmlElementSpans,
    element: XmlElement,
    deserializer: fn(&[u8], &XmlElementSpans) -> Result<T, Error>,
) -> Result<Option<T>, Error> {
    let all_spans = spans.get(element);
    if all_spans.len() >= 2 {
        debug!(
            "expected at most one {:?}, found {}",
            element,
            all_spans.len()
        );
    }
    match all_spans.first() {
        None => Ok(None),
        Some(x) => {
            let slice = &xml_document[x.start..x.end];
            let child_spans = extract_xml_element_spans(slice)?;
            deserializer(slice, &child_spans).map(Some)
        }
    }
}

pub fn collect_children<T: Send>(
    xml_document: &[u8],
    spans: &XmlElementSpans,
    element: XmlElement,
    deserializer: fn(&[u8], &XmlElementSpans) -> Result<T, Error>,
) -> Result<Vec<T>, Error> {
    spans
        .get(element)
        .into_par_iter()
        .map(|x| {
            let slice = &xml_document[x.start..x.end];
            let child_spans = extract_xml_element_spans(slice)?;
            deserializer(slice, &child_spans)
        })
        .collect()
}

/*#[derive(Clone)]
struct ElementContext {
    start_index: u64,
    name: Vec<u8>,
}

impl Debug for ElementContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.name))
    }
}*/

pub fn extract_xml_element_spans(xml_document: &[u8]) -> Result<XmlElementSpans, Error> {
    let mut reader = Reader::from_reader(xml_document);
    reader.config_mut().trim_text(true);

    let mut depth = 0;
    let mut element_spans: HashMap<XmlElement, Vec<Range<usize>>> = HashMap::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                // Skip grandchildren and deeper — only direct children of the root are recorded.
                if depth >= 2 {
                    reader.read_to_end(e.name())?;
                    continue;
                }
                depth += 1;
                // depth == 1 means this is the root element (the document being scanned).
                // Skip recording it; traversal continues into its children.
                if depth == 1 {
                    continue;
                }

                let gml_element = xml_element::XmlElement::from_local_name(e.local_name().as_ref());
                if let Some(x) = gml_element {
                    // buffer_position() is right after `>` of the start tag,
                    // so the `<` of the start tag is e.len() + 2 bytes back.
                    let pos_start = reader.buffer_position() as usize - e.len() - 2;
                    reader.read_to_end(e.name())?;
                    // buffer_position() is now right after `>` of the closing tag.
                    let pos_end = reader.buffer_position() as usize;

                    element_spans.entry(x).or_default().push(pos_start..pos_end);
                    depth -= 1;
                }
            }
            Ok(Event::Empty(e))
                // Self-closing elements (<foo/>) only appear as direct children (depth == 1).
                if depth == 1 => {
                    let gml_element =
                        xml_element::XmlElement::from_local_name(e.local_name().as_ref());
                    if let Some(x) = gml_element {
                        // buffer_position() is right after `>` of `<foo/>`,
                        // so `<` is e.len() + 3 bytes back (for `<`, `/`, `>`).
                        let pos_start = reader.buffer_position() as usize - e.len() - 3;
                        let pos_end = reader.buffer_position() as usize;
                        element_spans.entry(x).or_default().push(pos_start..pos_end);
                    }
                }
            Ok(Event::End(_)) => {
                depth -= 1;
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(Error::from(e)),
            _ => {}
        }
    }

    let xml_element_spans = XmlElementSpans::new(element_spans);
    Ok(xml_element_spans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_basic_abstract_thematic_surface() {
        let xml_document = b"<bldg:Building gml:id=\"DEBY_LOD2_59772\">
      <creationDate >2023-06-05T00:00:00+02:00</creationDate>
      <externalReference>
        <ExternalReference>
          <targetResource>DEBYvAAAAABTy70E</targetResource>
          <informationSystem>http://repository.gdi-de.org/schemas/adv/citygml/fdv/art.htm#_9100</informationSystem>
        </ExternalReference>
      </externalReference>
      <boundary>
        <con:WallSurface gml:id=\"DEBY_LOD2_59772_4becb506-d53b-44ca-a483-e6a3d238b4c2_2\">
          <lod2MultiSurface>
            <gml:MultiSurface srsDimension=\"3\">
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"DEBY_LOD2_59772_4becb506-d53b-44ca-a483-e6a3d238b4c2_2_poly\">
                  <gml:exterior>
                    <gml:LinearRing>
                      <gml:posList>691478.01 5334825.81 516.71 691478.01 5334825.81 528.425 691473.38 5334813.05 528.425 691473.38 5334813.05 516.71 691478.01 5334825.81 516.71</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>
          </lod2MultiSurface>
        </con:WallSurface>
      </boundary>
      <boundary>
        <con:WallSurface gml:id=\"DEBY_LOD2_59772_be3462c3-9865-467b-829d-76e6b9b692e7_2\">
          <lod2MultiSurface>
            <gml:MultiSurface srsDimension=\"3\">
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"DEBY_LOD2_59772_be3462c3-9865-467b-829d-76e6b9b692e7_2_poly\">
                  <gml:exterior>
                    <gml:LinearRing>
                      <gml:posList>691474.24 5334826.79 516.71 691474.24 5334826.79 528.425 691475.29 5334827.01 528.425 691475.29 5334827.01 516.71 691474.24 5334826.79 516.71</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>
          </lod2MultiSurface>
        </con:WallSurface>
      </boundary>
      <boundary>
        <con:WallSurface gml:id=\"DEBY_LOD2_59772_c0aae462-3f4b-4062-80bb-8cd04768ab1a_2\">
          <lod2MultiSurface>
            <gml:MultiSurface srsDimension=\"3\">
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"DEBY_LOD2_59772_c0aae462-3f4b-4062-80bb-8cd04768ab1a_2_poly\">
                  <gml:exterior>
                    <gml:LinearRing>
                      <gml:posList>691472.46 5334820.87 516.71 691472.46 5334820.87 528.425 691472.6 5334821.26 528.425 691472.6 5334821.26 516.71 691472.46 5334820.87 516.71</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>
          </lod2MultiSurface>
        </con:WallSurface>
      </boundary>
      <boundary>
        <con:RoofSurface gml:id=\"DEBY_LOD2_59772_a13e523a-7269-4637-88fa-e57eed6d9265_2\">
          <lod2MultiSurface>
            <gml:MultiSurface srsDimension=\"3\">
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"DEBY_LOD2_59772_a13e523a-7269-4637-88fa-e57eed6d9265_2_poly\">
                  <gml:exterior>
                    <gml:LinearRing>
                      <gml:posList>691470.72 5334813.99 528.425 691473.38 5334813.05 528.425 691478.01 5334825.81 528.425 691475.29 5334827.01 528.425 691474.24 5334826.79 528.425 691472.6 5334821.26 528.425 691472.46 5334820.87 528.425 691470.53 5334815.81 528.425 691470.29 5334815.2 528.425 691470.72 5334813.99 528.425</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>
          </lod2MultiSurface>
        </con:RoofSurface>
      </boundary>
      <boundary>
        <con:WallSurface gml:id=\"DEBY_LOD2_59772_bacdfeda-2181-42c2-ac94-bf086ec95291_2\">
          <lod2MultiSurface>
            <gml:MultiSurface srsDimension=\"3\">
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"DEBY_LOD2_59772_bacdfeda-2181-42c2-ac94-bf086ec95291_2_poly\">
                  <gml:exterior>
                    <gml:LinearRing>
                      <gml:posList>691470.72 5334813.99 516.71 691470.72 5334813.99 528.425 691470.29 5334815.2 528.425 691470.29 5334815.2 516.71 691470.72 5334813.99 516.71</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>
          </lod2MultiSurface>
        </con:WallSurface>
      </boundary>
      <boundary>
        <con:WallSurface gml:id=\"DEBY_LOD2_59772_e76604b3-3834-4420-a1e5-c660f32ab045_2\">
          <lod2MultiSurface>
            <gml:MultiSurface srsDimension=\"3\">
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"DEBY_LOD2_59772_e76604b3-3834-4420-a1e5-c660f32ab045_2_poly\">
                  <gml:exterior>
                    <gml:LinearRing>
                      <gml:posList>691470.29 5334815.2 516.71 691470.29 5334815.2 528.425 691470.53 5334815.81 528.425 691470.53 5334815.81 516.71 691470.29 5334815.2 516.71</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>
          </lod2MultiSurface>
        </con:WallSurface>
      </boundary>
      <boundary>
        <con:WallSurface gml:id=\"DEBY_LOD2_59772_c8be8650-e40d-4c7d-b491-d892085763aa_2\">
          <lod2MultiSurface>
            <gml:MultiSurface srsDimension=\"3\">
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"DEBY_LOD2_59772_c8be8650-e40d-4c7d-b491-d892085763aa_2_poly\">
                  <gml:exterior>
                    <gml:LinearRing>
                      <gml:posList>691473.38 5334813.05 516.71 691473.38 5334813.05 528.425 691470.72 5334813.99 528.425 691470.72 5334813.99 516.71 691473.38 5334813.05 516.71</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>
          </lod2MultiSurface>
        </con:WallSurface>
      </boundary>
      <boundary>
        <con:WallSurface gml:id=\"DEBY_LOD2_59772_30ce7949-9c18-4a98-bdae-afeb9a0b6252_2\">
          <lod2MultiSurface>
            <gml:MultiSurface srsDimension=\"3\">
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"DEBY_LOD2_59772_30ce7949-9c18-4a98-bdae-afeb9a0b6252_2_poly\">
                  <gml:exterior>
                    <gml:LinearRing>
                      <gml:posList>691472.6 5334821.26 516.71 691472.6 5334821.26 528.425 691474.24 5334826.79 528.425 691474.24 5334826.79 516.71 691472.6 5334821.26 516.71</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>
          </lod2MultiSurface>
        </con:WallSurface>
      </boundary>
      <boundary>
        <con:WallSurface gml:id=\"DEBY_LOD2_59772_27c754d1-d65b-4b5e-a17a-8a24080009e1_2\">
          <lod2MultiSurface>
            <gml:MultiSurface srsDimension=\"3\">
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"DEBY_LOD2_59772_27c754d1-d65b-4b5e-a17a-8a24080009e1_2_poly\">
                  <gml:exterior>
                    <gml:LinearRing>
                      <gml:posList>691475.29 5334827.01 516.71 691475.29 5334827.01 528.425 691478.01 5334825.81 528.425 691478.01 5334825.81 516.71 691475.29 5334827.01 516.71</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>
          </lod2MultiSurface>
        </con:WallSurface>
      </boundary>
      <boundary>
        <con:GroundSurface gml:id=\"DEBY_LOD2_59772_bae63dbe-80b8-4e3a-a78e-755230366512_2\">
          <lod2MultiSurface>
            <gml:MultiSurface srsDimension=\"3\">
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"DEBY_LOD2_59772_bae63dbe-80b8-4e3a-a78e-755230366512_2_poly\">
                  <gml:exterior>
                    <gml:LinearRing>
                      <gml:posList>691470.29 5334815.2 516.71 691470.53 5334815.81 516.71 691472.46 5334820.87 516.71 691472.6 5334821.26 516.71 691474.24 5334826.79 516.71 691475.29 5334827.01 516.71 691478.01 5334825.81 516.71 691473.38 5334813.05 516.71 691470.72 5334813.99 516.71 691470.29 5334815.2 516.71</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>
          </lod2MultiSurface>
        </con:GroundSurface>
      </boundary>
      <boundary>
        <con:WallSurface gml:id=\"DEBY_LOD2_59772_c21df86c-d952-4aa9-a5e2-42418b07a2a9_2\">
          <lod2MultiSurface>
            <gml:MultiSurface srsDimension=\"3\">
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"DEBY_LOD2_59772_c21df86c-d952-4aa9-a5e2-42418b07a2a9_2_poly\">
                  <gml:exterior>
                    <gml:LinearRing>
                      <gml:posList>691470.53 5334815.81 516.71 691470.53 5334815.81 528.425 691472.46 5334820.87 528.425 691472.46 5334820.87 516.71 691470.53 5334815.81 516.71</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>
          </lod2MultiSurface>
        </con:WallSurface>
      </boundary>
      <bldg:function>31001_9998</bldg:function>
      <bldg:roofType>1000</bldg:roofType>
      <bldg:storeysAboveGround>3</bldg:storeysAboveGround>
    </bldg:Building>";

        let xml_element_spans = extract_xml_element_spans(xml_document).expect("should work");

        let boundary_spans = xml_element_spans.get(XmlElement::BoundaryProperty);
        assert_eq!(boundary_spans.len(), 11);

        /*for (gml_element, spans) in gml_element_spans {
            for current_span in spans {
                let snippet = &xml_document[current_span.start..current_span.end];
                let a = String::from_utf8_lossy(snippet);

                println!("abc: {:?}", a);
            }
        }*/
    }

    #[test]
    fn test_deserialize_basic() {
        let xml_document = b"<bldg:Building gml:id=\"DEBY_LOD2_59772\">
      <creationDate >2023-06-05T00:00:00+02:00</creationDate>
      <externalReference>
        <ExternalReference>
          <targetResource>DEBYvAAAAABTy70E</targetResource>
          <informationSystem>http://repository.gdi-de.org/schemas/adv/citygml/fdv/art.htm#_9100</informationSystem>
        </ExternalReference>
      </externalReference>
      <boundary xlink:href=\"#DEBY_LOD2_59772_4becb506-d53b-44ca-a483-e6a3d238b4c2_2\"></boundary>
    </bldg:Building>";

        let xml_element_spans = extract_xml_element_spans(xml_document).expect("should work");

        let boundary_span = xml_element_spans
            .first(XmlElement::BoundaryProperty)
            .unwrap();
        let xml_snippet = &xml_document[boundary_span.start..boundary_span.end];
        let xml_snippet_utf = String::from_utf8_lossy(xml_snippet);

        assert_eq!(
            xml_snippet_utf,
            "<boundary xlink:href=\"#DEBY_LOD2_59772_4becb506-d53b-44ca-a483-e6a3d238b4c2_2\"></boundary>"
        );
    }

    #[test]
    fn test_deserialize_with_tw() {
        let xml_document = b"<cityObjectMember>
        <bldg:Building gml:id=\"DEBY_LOD2_59772\">
        <creationDate >2023-06-05T00:00:00+02:00</creationDate>
        <boundary xlink:href=\"#DEBY_LOD2_59772_4becb506-d53b-44ca-a483-e6a3d238b4c2_2\"></boundary>
    </bldg:Building>
    </cityObjectMember>";

        let xml_element_spans = extract_xml_element_spans(xml_document).expect("should work");

        assert_eq!(xml_element_spans.get(XmlElement::BoundaryProperty).len(), 0);
    }
}
