//! Provide generated UCI v2.5 helper types for the supported UCI message subset.
//!
//! This file is generated from the local UCI v2.5 XSD subset.
//! Generator: scripts/generate_uci_types.py, adapted 2026-10-04/05 from
//! SLEET e3c2d798a229766571ed7be1f5b28d2dcdb51868 (Apache-2.0).
//! Schema: UCI 002.5.0, revision 73a286fc4b881d9f328dbd64148525606a92c33a.
//! UCI artifacts retain separate Government Owned terms; see LICENSES/uci-schema.

#![allow(clippy::all, clippy::cargo, clippy::nursery, clippy::pedantic)]

use serde::{Deserialize, Serialize};

/// CVEnumISMatomicEnergyMarkings Values
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AtomicEnergyMarkingsEnum {
    /// RESTRICTED DATA
    #[serde(rename = "RD")]
    Rd,
    /// RD-CRITICAL NUCLEAR WEAPON DESIGN INFORMATION
    #[serde(rename = "RD_CNWDI")]
    RdCnwdi,
    /// DOE Order 452.7 Restricted Data Sigma 14. Category of sensitive information, including bypass
    /// scenarios, concerning the vulnerability of nuclear weapons to a deliberate, unauthorized nuclear
    /// detonation or to the denial of authorized use.
    #[serde(rename = "RD_SG_14")]
    RdSg14,
    /// DOE Order 452.7 Restricted Data Sigma 15. Category of sensitive information concerning the
    /// design and function of nuclear weapon use control systems, features, and components.
    #[serde(rename = "RD_SG_15")]
    RdSg15,
    /// DOE Order 452.8 Restricted Data Sigma 18.Category of Nuclear Weapon Data including information
    /// that allows or significantly facilitates a nation or entity to fabricate a credible nuclear
    /// weapon or nuclear explosive device based on a proven, certified, or endorsed U.S. nuclear weapon
    /// or device.
    #[serde(rename = "RD_SG_18")]
    RdSg18,
    /// DOE Order 457.1A Restricted Data Sigma 20. Category of Nuclear Weapon Data that pertains to
    /// "crude, simple, or innovative" improvised nuclear device (IND) designs, concepts, and related
    /// manufacturing or processing pathways.
    #[serde(rename = "RD_SG_20")]
    RdSg20,
    /// FORMERLY RESTRICTED DATA
    #[serde(rename = "FRD")]
    Frd,
    /// DOE Order 452.7 Formerly Restricted Data Sigma 14. Category of sensitive information, including
    /// bypass scenarios, concerning the vulnerability of nuclear weapons to a deliberate, unauthorized
    /// nuclear detonation or to the denial of authorized use.
    #[serde(rename = "FRD_SG_14")]
    FrdSg14,
    /// DOE Order 452.7 Formerly Restricted Data Sigma 15. Category of sensitive information concerning
    /// the design and function of nuclear weapon use control systems, features, and components.
    #[serde(rename = "FRD_SG_15")]
    FrdSg15,
    /// DOE Order 452.8 Formerly Restricted Data Sigma 18.Category of Nuclear Weapon Data including
    /// information that allows or significantly facilitates a nation or entity to fabricate a credible
    /// nuclear weapon or nuclear explosive device based on a proven, certified, or endorsed U.S.
    /// nuclear weapon or device.
    #[serde(rename = "FRD_SG_18")]
    FrdSg18,
    /// DOE Order 457.1A Formerly Restricted Data Sigma 20. Category of Nuclear Weapon Data that
    /// pertains to "crude, simple, or innovative" improvised nuclear device (IND) designs, concepts,
    /// and related manufacturing or processing pathways.
    #[serde(rename = "FRD_SG_20")]
    FrdSg20,
    /// DoD CONTROLLED NUCLEAR INFORMATION
    #[serde(rename = "DCNI")]
    Dcni,
    /// DoE CONTROLLED NUCLEAR INFORMATION
    #[serde(rename = "UCNI")]
    Ucni,
    /// TRANSCLASSIFIED FOREIGN NUCLEAR INFORMATION
    #[serde(rename = "TFNI")]
    Tfni,
}

/// (U) All currently valid CUI Basic markings from the National Archives. This enum is used by
/// CUI_Basic. PERMISSIBLE VALUES The permissible values for this simple type are defined in the
/// Controlled Value Enumeration: CVEnumISMCUIBasic.xml
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CuiBasicEnum {
    /// Administrative Proceedings. Adjudication of agency-related matters including, but not limited
    /// to, dispute resolution, settlements, and issuances of orders.
    #[serde(rename = "ADPO")]
    Adpo,
    /// Agriculture. Information related to the agricultural operation, farming or conservation
    /// practices, or the actual land of an agricultural producer or landowner.
    #[serde(rename = "AG")]
    Ag,
    /// Asylee. Related to refugee applications and associated hearings to grant asylum to foreign
    /// nationals in the United States to be recognized as asylees.
    #[serde(rename = "ASYL")]
    Asyl,
    /// Bank Secrecy. Information that is provided to the government pursuant to the Bank Secrecy Act,
    /// including but not limited to, suspicious activity reports (SAR), currency transaction reports
    /// (CTR), reports of international transportation of currency or monetary instruments (CMIR),
    /// reports of cash payment over $10,000 received in trade or business, and reports of foreign bank
    /// and financial accounts (FBAR). Reports filed under the Bank Secrecy Act (BSA), codified in
    /// relevant part at 31 U.S.C. § 5311 et seq, are specifically exempt from disclosure under the
    /// Freedom of Information Act, codified at 5 U.S.C. § 552, and also may not be disclosed under any
    /// State, local, tribal, or territorial "freedom of information," "open government," or similar
    /// law. See 31 U.S.C. § 5319; 5 U.S.C. § 552(b)(3). These reports (BSA Reports), are maintained in
    /// a system of records containing information compiled for law enforcement investigative purposes
    /// that has been exempted from the access provisions of the Privacy Act in accordance with 5 U.S.C.
    /// §§ 552a(j)(2) and (k)(2). BSA Reports may only be re-disseminated in strict accordance with
    /// guidelines established by the Financial Crimes Enforcement Network (FinCEN), the Treasury bureau
    /// that administers the BSA. Suspicious Activity Reports, one of the types of required reports
    /// filed under the BSA, are required to be kept confidential in accordance with 31 U.S.C. §
    /// 5318(g)(2) and implementing regulations. To the extent information falling under the purview of
    /// the BSA is collected, accessed, or used for any Federal tax administration purpose, it is also
    /// subject to the confidentiality provisions of the Internal Revenue Code, codified at 26 U.S.C. §
    /// 6103.
    #[serde(rename = "FSEC")]
    Fsec,
    /// Battered Spouse or Child. Related to information within applications and associated hearings
    /// provided by, or that could identify, a battered spouse or child of a US citizen or US permanent
    /// resident seeking independent protected status within the United States.
    #[serde(rename = "BATT")]
    Batt,
    /// Chemical-terrorism Vulnerability Information. In accordance with Section 550(c) of the
    /// Department of Homeland Security Appropriations Act of 2007, the following information, whether
    /// transmitted verbally, electronically, or in written form, shall constitute CVI, see (1) - (9).
    #[serde(rename = "CVI")]
    Cvi,
    /// Child Victim/Witness. Information pertaining to a minor who was a victim, witness, or potential
    /// witness to a criminal act.
    #[serde(rename = "CVIC")]
    Cvic,
    /// Collective Bargaining. Defining agencies' and representatives' duty to negotiate in good faith
    /// to include disclosure of certain labor relations training and guidance materials and limiting
    /// the issuance of certain subpoenas.
    #[serde(rename = "BARG")]
    Barg,
    /// Committed Person. Related to information concerning the mental condition of a person committed
    /// to a psychiatric facility.
    #[serde(rename = "CMPRS")]
    Cmprs,
    /// Communications. Related to the contents of any wire, oral, or electronic communication.
    #[serde(rename = "LCOMM")]
    Lcomm,
    /// Comptroller General. Concerning the Officer of the United States Government who is charged with
    /// duties relating to fiscal affairs, including auditing, examining accounts, and reporting the
    /// financial status of the United States Government.
    #[serde(rename = "COMPT")]
    Compt,
    /// Death Records. Related to information contained within an official document issued by a public
    /// registry verifying that a person has died, with information such as the date and time of death,
    /// the cause of death, and the signature of the attending or examining physician.
    #[serde(rename = "DREC")]
    Drec,
    /// DoD Critical Infrastructure Security Information. Information that, if disclosed, would reveal
    /// vulnerabilities in the DoD critical infrastructure and, if exploited, would likely result in the
    /// significant disruption, destruction, or damage of or to DoD operations, property, or facilities,
    /// including information regarding the securing and safeguarding of explosives, hazardous
    /// chemicals, or pipelines, related to critical infrastructure or protected systems owned or
    /// operated on behalf of the DoD, including vulnerability assessments prepared by or on behalf of
    /// the DoD, explosives safety information (including storage and handling), and other site-specific
    /// information on or relating to installation security.
    #[serde(rename = "DCRIT")]
    Dcrit,
    /// Electronic Funds Transfer. Relating to the computer-based systems used to perform financial
    /// transactions electronically.
    #[serde(rename = "XFER")]
    Xfer,
    /// Emergency Management. Related to information concerning the continuity of executive branch
    /// operations during all-hazards emergencies or other situations that may disrupt normal
    /// operations.
    #[serde(rename = "EMGT")]
    Emgt,
    /// Export Controlled. Unclassified information concerning certain items, commodities, technology,
    /// software, or other information whose export could reasonably be expected to adversely affect the
    /// United States national security and nonproliferation objectives. To include dual use items;
    /// items identified in export administration regulations, international traffic in arms regulations
    /// and the munitions list; license applications; and sensitive nuclear technology information.
    #[serde(rename = "EXPT")]
    Expt,
    /// Export Controlled Research. Related to the systematic investigation into and study of materials
    /// and sources in order to establish facts and reach new conclusions.
    #[serde(rename = "EXPTR")]
    Exptr,
    /// Federal Grand Jury. Material obtained pursuant to a federal grand jury subpoena, which includes
    /// (1) any reference to a specific sitting grand jury; (2) any documentation or data obtained by a
    /// grand jury subpoena if disclosure of such material tends to reveal what transpired before or at
    /// the direction of the federal grand jury; (3) documentation prepared specifically for the federal
    /// grand jury; and (4) transcripts or other recordings of testimony presented to the federal grand
    /// jury.
    #[serde(rename = "JURY")]
    Jury,
    /// Federal Housing Finance Non-Public Information. Related to information that the Federal Housing
    /// Finance Agency (FHFA) has not made public that is created by, obtained by, or communicated to an
    /// FHFA employee in connection with the performance of official duties, regardless of who is in
    /// possession of the information, including confidential supervisory information. Confidential
    /// supervisory information includes FHFA reports of examination, inspection and visitation,
    /// confidential operating and condition reports, and any information derived from, related to, or
    /// contained in such reports, or gathered by FHFA in the course of any investigation, suspicious
    /// activity report, cease-and- desist order, civil money penalty enforcement order, suspension,
    /// removal or prohibition order, or other supervisory or enforcement orders or actions taken under
    /// the Federal Housing Enterprises Financial Safety and Soundness Act of 1992, and other conditions
    /// as set forth in 12 CFR Part 1214.1.
    #[serde(rename = "FHFANPI")]
    Fhfanpi,
    /// Financial Supervision Information. Related to information connected to an agency's
    /// responsibilities to supervise, examine, and evaluate a financial institution.
    #[serde(rename = "FSI")]
    Fsi,
    /// General Critical Infrastructure Information. Systems and assets, whether physical or virtual, so
    /// vital that the incapacity or destruction of such may have a debilitating impact on the security,
    /// economy, public health or safety, environment, or any combination of these matters, across any
    /// Federal, State, regional, territorial, or local jurisdiction.
    #[serde(rename = "CRIT")]
    Crit,
    /// General Financial Information. Related to the duties, transactions, or otherwise falling under
    /// the purview of financial institutions or United States Government fiscal functions. Uses may
    /// include, but are not limited to, customer information held by a financial institution.
    #[serde(rename = "FNC")]
    Fnc,
    /// General Intelligence. Related to intelligence activities, sources, or methods.
    #[serde(rename = "INTEL")]
    Intel,
    /// General Law Enforcement. Related to techniques and procedures for law enforcement operations,
    /// investigations, prosecutions, or enforcement actions.
    #[serde(rename = "LEI")]
    Lei,
    /// General Nuclear. Related to protection of information concerning nuclear reactors, materials, or
    /// security.
    #[serde(rename = "NUC")]
    Nuc,
    /// General Privacy. Refers to personal information, or, in some cases, "personally identifiable
    /// information," as defined in OMB M-17-12, or "means of identification" as defined in 18 USC
    /// 1028(d)(7).
    #[serde(rename = "PRVCY")]
    Prvcy,
    /// General Proprietary Business Information. Material and information relating to, or associated
    /// with, a company's products, business, or activities, including but not limited to financial
    /// information; data or statements; trade secrets; product research and development; existing and
    /// future product designs and performance specifications.
    #[serde(rename = "PROPIN")]
    Propin,
    /// Genetic Information. The term "genetic information" means, with respect to any individual,
    /// information about-- (i) such individual's genetic tests, (ii) the genetic tests of family
    /// members of such individual, and (iii) the manifestation of a disease or disorder in family
    /// members of such individual.
    #[serde(rename = "GENETIC")]
    Genetic,
    /// Health Information. As per 42 USC 1320d(4), "health information" means any information, whether
    /// oral or recorded in any form or medium, that (A) is created or received by a health care
    /// provider, health plan, public health authority, employer, life insurer, school or university, or
    /// health care clearinghouse; and (B) relates to the past, present, or future physical or mental
    /// health or condition of an individual, the provision of health care to an individual, or the
    /// past, present, or future payment for the provision of health care to an individual.
    #[serde(rename = "HLTH")]
    Hlth,
    /// Provisional - Homeland Security Agreement Information. Information DHS receives and is required
    /// to protect pursuant to an agreement with state, local, tribal, territorial, and private sector
    /// partners. DHS receives this information in furtherance of the missions of the Department,
    /// including but not limited to, support of the Fusion Center Initiative and activities for cyber
    /// information sharing consistent with the Cybersecurity Information Security Act.
    #[serde(rename = "CUI_PROVISIONAL_HSAI")]
    CuiProvisionalHsai,
    /// Provisional - Homeland Security Enforcement Information. Unclassified information of a sensitive
    /// nature lawfully created, possessed, or transmitted by DHS in furtherance of its immigration,
    /// customs, and other civil and criminal enforcement missions, the unauthorized disclosure of which
    /// could adversely impact the mission of the Department.
    #[serde(rename = "CUI_PROVISIONAL_HSEI")]
    CuiProvisionalHsei,
    /// Informant. Related to the identity of a human source.
    #[serde(rename = "INF")]
    Inf,
    /// Information Systems Vulnerability Information. Related to information that if not protected,
    /// could result in adverse effects to information systems. Information system means a discrete set
    /// of information resources organized for the collection, processing, maintenance, use, sharing,
    /// dissemination, or disposition of information.
    #[serde(rename = "ISVI")]
    Isvi,
    /// Provisional - Information Systems Vulnerability Information - Homeland. a. DHS information
    /// technology internal systems data revealing infrastructure used for servers, desktops, and
    /// networks; applications name, version and release; switching, router, and gateway information;
    /// interconnections and access methods; mission or business use/need. Examples of information are
    /// systems inventories and enterprise architecture models. Information pertaining to national
    /// security systems and eligible for classification under Executive Order 13526, will be classified
    /// as appropriate. b. Information regarding developing or current technology, the release of which
    /// could hinder the objectives of DHS, compromise a technological advantage or countermeasure,
    /// cause a denial of service, or provide an adversary with sufficient information to close,
    /// counterfeit, or circumvent a process or system.
    #[serde(rename = "CUI_PROVISIONAL_ISVIH")]
    CuiProvisionalIsvih,
    /// Inspector General Protected. Related to the identity of a person making a report to the
    /// Inspector General of any Executive agency.
    #[serde(rename = "PRIIG")]
    Priig,
    /// Internal Data. Refers to a category of information that is not intended to be disseminated
    /// beyond CIA channels that involves intelligence activities, sources, or methods. This information
    /// may also relate to the CIA's organization, functions, names, official titles, salaries, or
    /// numbers of personnel.
    #[serde(rename = "ID")]
    Id,
    /// Provisional - International Agreement Information - Homeland. Information DHS receives and is
    /// required to protect pursuant to an information sharing agreement or arrangement with a foreign
    /// government, an international organization of governments or any element thereof, an
    /// international or foreign public or judicial body, or an international or foreign private or
    /// non-governmental organization.
    #[serde(rename = "CUI_PROVISIONAL_IAIH")]
    CuiProvisionalIaih,
    /// International Financial Institutions. Relating to entities that provide financial services for
    /// its clients or members, and were established (or chartered) by more than one country, and hence
    /// are subjects of international law.
    #[serde(rename = "FINT")]
    Fint,
    /// Inventions. An invention is any art or process (way of doing or making things), machine,
    /// manufacture, design, or composition of matter, or any new and useful improvement thereof, or any
    /// variety of plant, which is or may be patentable under the patent laws of the United States, in
    /// which the federal government owns or may own a right, title, or interest.
    #[serde(rename = "INVENT")]
    Invent,
    /// Investigation. Related to information obtained during the course of a law enforcement
    /// investigation or action, civil or criminal.
    #[serde(rename = "INV")]
    Inv,
    /// Investment Survey. Information reported to Treasury, the Federal Reserve Board of Governors, or
    /// the Federal Reserve Banks as part of the Treasury International Capital (TIC) data reporting
    /// system.
    #[serde(rename = "SURV")]
    Surv,
    /// Juvenile. Related to the identity of individual juvenile youths.
    #[serde(rename = "JUV")]
    Juv,
    /// Legal Privilege. Per 12 USC 78x: The term "privilege" includes any work-product privilege,
    /// attorney-client privilege, governmental privilege, or other privilege recognized under Federal,
    /// State, or foreign law. Per 502(g): (1) "attorney-client privilege" means the protection that
    /// applicable law provides for confidential attorney-client communications; and (2) "work-product
    /// protection" means the protection that applicable law provides for tangible material (or its
    /// intangible equivalent) prepared in anticipation of litigation or for trial.
    #[serde(rename = "PRIVILEGE")]
    Privilege,
    /// Legislative Materials. Data related to Congress's legislative, investigatory or oversight
    /// responsibilities of the Executive branch of the Federal government. This includes data related
    /// to proposed or pending legislation as well as inquiries submitted by Congress to Federal
    /// agencies, agency responses to those inquiries and any other information which, if disclosed,
    /// would reveal the nature and scope of Congressional inquiries.
    #[serde(rename = "LMI")]
    Lmi,
    /// Mergers. Relating to methods by which corporations legally unify ownership of assets formerly
    /// subject to separate controls.
    #[serde(rename = "MERG")]
    Merg,
    /// Military Personnel Records. Any member or former member of the armed forces or affiliated
    /// organization of the Department of Defense.
    #[serde(rename = "MIL")]
    Mil,
    /// National Security Letter. Related to administrative orders sent to compel the recipients of the
    /// letters to provide information to federal investigators.
    #[serde(rename = "LNSL")]
    Lnsl,
    /// Net Worth. Related to the net worth of an individual and/or their affiliates in certain
    /// administrative proceedings.
    #[serde(rename = "NETW")]
    Netw,
    /// Naval Nuclear Propulsion Information. Related to the safety of reactors and associated naval
    /// nuclear propulsion plants, and control of radiation and radioactivity associated with naval
    /// nuclear propulsion activities, including prescribing and enforcing standards and regulations for
    /// these areas as they affect the environment and the safety and health of workers, operators, and
    /// the general public.
    #[serde(rename = "NNPI")]
    Nnpi,
    /// Critical information determined to give evidence of the planning and execution of sensitive
    /// (frequently classified) government activities after going through a formal systematic vetting
    /// process in accordance with National Security Decision Directive Number 298. This process
    /// identifies unclassified information that must be protected. It almost always results from an
    /// agency's official OPSEC program, or is otherwise commonly approved for use by the CUI Senior
    /// Agency Official.
    #[serde(rename = "OPSEC")]
    Opsec,
    /// Nuclear Recommendation Material. Related to recommendations to the Secretary of Energy with
    /// respect to Department of Energy defense nuclear facilities as determined necessary to ensure
    /// adequate protection of public health and safety.
    #[serde(rename = "RECCOM")]
    Reccom,
    /// Nuclear Security-Related Information. Related to information that could be useful, or could
    /// reasonably be expected to be useful, to a terrorist in a potential attack that does not qualify
    /// as Safeguards or classified information, including the exact location and quantities of
    /// radioactive material, certain detailed design drawings, information on nearby facilities,
    /// emergency planning information, and certain assessments of vulnerability and safety analyses.
    #[serde(rename = "SRI")]
    Sri,
    /// Ocean Common Carrier and Marine Terminal Operator Agreements. Relating to agreements between or
    /// among ocean common carriers and marine terminal operators as referenced in 46 USC 40301 and
    /// 40306.
    #[serde(rename = "OCCMTO")]
    Occmto,
    /// Ocean Common Carrier Service Contracts. Relating to an agreement for the provision of services
    /// filed with the Federal Maritime Commission as referenced in 46 USC 40502(b), 46 CFR 530.4,
    /// and/or 46 CFR 531.4(a).
    #[serde(rename = "SERV")]
    Serv,
    /// Provisional - Operations Security Information. Unclassified information that could constitute an
    /// indicator of U.S. Government intentions, capabilities, operations, or activities or otherwise
    /// threaten/compromise operations security.
    #[serde(rename = "CUI_PROVISIONAL_OSI")]
    CuiProvisionalOsi,
    /// Patent Applications. Application for patent filed under 35 U.S.C. 111(a) that includes all types
    /// of patent applications (i.e., utility, design, plant, and reissue) except provisional
    /// applications. The nonprovisional application establishes the filing date and initiates the
    /// examination process. A nonprovisional utility patent application must include a specification,
    /// including a claim or claims; drawings, when necessary; an oath or declaration; and the
    /// prescribed filing fee.
    #[serde(rename = "APP")]
    App,
    /// Pen Register/Trap and Trace. Related to devices used to identify incoming and outgoing telephone
    /// numbers.
    #[serde(rename = "TRACE")]
    Trace,
    /// Permanent Resident Status. Related to applications and associated hearings to grant permanent
    /// residency to foreign nationals living in the United States.
    #[serde(rename = "RESD")]
    Resd,
    /// Personnel Records. Related to the employees of federal agencies.
    #[serde(rename = "PERS")]
    Pers,
    /// Provisional - Personnel Security Information. Information that could result in physical risk to
    /// DHS personnel or other individuals that DHS is responsible for protecting.
    #[serde(rename = "CUI_PROVISIONAL_PSI")]
    CuiProvisionalPsi,
    /// Pesticide Producer Survey. Related to the data gathered regarding the production of pesticides
    /// that specifies the non-disclosure of the identity and location of individual producers.
    #[serde(rename = "PEST")]
    Pest,
    /// Physical Security. Related to protection of federal buildings, grounds or property.
    #[serde(rename = "PHYS")]
    Phys,
    /// Provisional - Physical Security - Homeland. Assessments or reports illustrating or disclosing
    /// facility infrastructure or security vulnerabilities related to the protection of federal
    /// buildings, grounds, or property, such as threat assessments, system security plans, contingency
    /// plans, risk management plans, business impact analysis studies, and certification and
    /// accreditation documentation.
    #[serde(rename = "CUI_PROVISIONAL_PHYSH")]
    CuiProvisionalPhysh,
    /// Presentence Report. A report, generally prepared to assist the court in determining the most
    /// appropriate sentence for a defendant. It can include an assessment of the nature and seriousness
    /// of the offense and should contain details summarizing the background information of the
    /// defendant and the crime.
    #[serde(rename = "PRE")]
    Pre,
    /// Prior Arrest. Information related to previous instances of law enforcement official's
    /// apprehension and formal processing of a suspect.
    #[serde(rename = "PRIOR")]
    Prior,
    /// Provisional - Privacy Information. Information referred to as Personally Identifiable
    /// Information (PII). PII embodies information that can be used to distinguish or trace an
    /// individual's identity, either alone or when combined with other information that is linked or
    /// linkable to a specific individual.
    #[serde(rename = "CUI_PROVISIONAL_PRIVACY")]
    CuiProvisionalPrivacy,
    /// Proprietary Postal. Concerning or related to the course of business of the United States Postal
    /// Service.
    #[serde(rename = "POST")]
    Post,
    /// Protective Order. Stipulation that certain information that would normally fall under discovery
    /// rules will not be disclosed for specifically stated reason.
    #[serde(rename = "LPROT")]
    Lprot,
    /// Railroad Safety Analysis Records. Related to the establishment, implementation, or modification
    /// of a railroad safety risk reduction program or pilot program, if the record is: (1) Supplied to
    /// the Secretary (of Transportation) pursuant to that safety risk reduction program or pilot
    /// program; or (2) made available for inspection and copying by an officer, employee, or agent of
    /// the Secretary pursuant to that safety risk reduction program or pilot program.
    #[serde(rename = "RAIL")]
    Rail,
    /// Retirement. Related to post-employment funding provided by an employer.
    #[serde(rename = "RTR")]
    Rtr,
    /// Reward. Related to the identity of a recipient of a reward.
    #[serde(rename = "RWRD")]
    Rwrd,
    /// SAFETY Act Information. Defined as "SAFETY Act Confidential Information" in 6 CFR Part 25, the
    /// regulations implementing the Support Anti-terrorism by Fostering Effective Technologies Act of
    /// 2002, SAFETY Act Information includes any and all information and data voluntarily submitted to
    /// the Department of Homeland Security under this part (including Applications, Pre-Applications,
    /// other forms, supporting documents and other materials relating to any of the foregoing, and
    /// responses to requests for additional information), including, but not limited to, inventions,
    /// devices, Technology, know-how, designs, copyrighted information, trade secrets, confidential
    /// business information, analyses, test and evaluation results, manuals, videotapes, contracts,
    /// letters, facsimile transmissions, electronic mail and other correspondence, financial
    /// information and projections, actuarial calculations, liability estimates, insurance quotations,
    /// and business and marketing plans.
    #[serde(rename = "SAFE")]
    Safe,
    /// Secrecy Orders. An order by the Commissioner of Patents that an invention be kept secret and to
    /// withhold the publication of an application or the grant of a patent due to national security
    /// concerns.
    #[serde(rename = "PSEC")]
    Psec,
    /// Provisional - Sensitive Personally Identifiable Information. A subset of PII that, if lost,
    /// compromised or disclosed without authorization could result in substantial harm, embarrassment,
    /// inconvenience, or unfairness to an individual. Some forms of PII are sensitive as stand-alone
    /// elements. a. Examples of stand-alone PII include: Social Security Numbers (SSN), driver's
    /// license or state identification number; Alien Registration Numbers; financial account number;
    /// and biometric identifiers such as fingerprint, voiceprint, or iris scan. b. Additional examples
    /// of SPII include any groupings of information that contain an individual's name or other unique
    /// identifier plus one or more of the following elements: Truncated SSN (such as last four digits)
    /// Date of birth (month, day, and year) Citizenship or immigration status Ethnic or religious
    /// affiliation Sexual orientation Criminal history Medical information System authentication
    /// information such as mother's maiden name, account passwords, or personal identification numbers
    /// c. Other PII may be "sensitive" depending on its context, such in as a list of employees and
    /// their performance rating(s) or an unlisted home address or phone number. In contrast, a business
    /// card or public telephone directory of agency employees contains PII, but is not sensitive.
    #[serde(rename = "CUI_PROVISIONAL_PII")]
    CuiProvisionalPii,
    /// Sex Crime Victim. Related to the identity of a victim of a sex offense.
    #[serde(rename = "SCV")]
    Scv,
    /// Small Business Research and Technology. Relating to certain "Small Business Innovation Research
    /// Program" and "Small Business Technology Transfer Program" information in a government database,
    /// as referenced in 15 USC 638(k)(2).
    #[serde(rename = "SBIZ")]
    Sbiz,
    /// Source Selection. Per FAR 2.101: any of the following information that is prepared for use by an
    /// agency for the purpose of evaluating a bid or proposal to enter into an agency procurement
    /// contract, if that information has not been previously made available to the public or disclosed
    /// publicly: (Items 1-10).
    #[serde(rename = "SSEL")]
    Ssel,
    /// Statistical Information. Refers to information collected by a Federal statistical agency, unit,
    /// or program for statistical purposes or used for statistical activities; under law, regulation,
    /// or Government-wide policy such 'Statistical' CUI requires: (1) protection from unauthorized
    /// disclosure; (2) special handling safeguards; and/or (3) prescribed limits on access or
    /// dissemination.
    #[serde(rename = "STAT")]
    Stat,
    /// Status Adjustment. Related to applications for the adjustment of immigration status.
    #[serde(rename = "ADJ")]
    Adj,
    /// Student Records. As per 20 USC 1232g, the Family Educational Rights and Privacy Act of 1974, an
    /// education record which is comprised of those records which are directly related to a student.
    #[serde(rename = "STUD")]
    Stud,
    /// System for Award Management. Relating to the primary United States Government system for
    /// contractor registration and awards.
    #[serde(rename = "CONREG")]
    Conreg,
    /// Tax Convention. Related to any-- (A) agreement entered into with the competent authority of one
    /// or more foreign governments pursuant to a tax convention, (B) application for relief under a tax
    /// convention, (C) background information related to such agreement or application, (D) document
    /// implementing such agreement, and (E) other information exchanged pursuant to a tax convention
    /// which is treated as confidential or secret under the tax convention. Tax convention information
    /// originating with the IRS generally retains its confidential status even when it resides with
    /// agencies other than the IRS.
    #[serde(rename = "CONV")]
    Conv,
    /// Taxpayer Advocate Information. A local taxpayer advocate (LTA) has the discretion to not
    /// disclose to the IRS contact with, or information provided by, a taxpayer. Such discretion may
    /// result in declinations of requests for information by IRS personnel including cases involving
    /// criminal tax investigations or those where the failure to provide information to the IRS would
    /// be beneficial to the taxpayer but to the detriment of the IRS. Such discretion does not extend
    /// to cases where the LTA believes a taxpayer is using the Taxpayer Advocate's office to perpetuate
    /// a fraud on the government or in cases where the information is sought in litigation.
    #[serde(rename = "TAI")]
    Tai,
    /// Temporary Protected Status. Related to findings that conditions in a given country pose a danger
    /// to personal safety due to ongoing armed conflict or an environmental disaster and persons should
    /// receive special temporary status to remain in the United States.
    #[serde(rename = "PROT")]
    Prot,
    /// Terrorist Screening. Related to information gathering and analysis concerning possible threats
    /// or acts of a destructive nature.
    #[serde(rename = "LSCRN")]
    Lscrn,
    /// Unclassified Controlled Nuclear Information - Defense. Relating to Department of Defense special
    /// nuclear material (SNM), equipment, and facilities, as defined by 32 CFR 223.
    #[serde(rename = "DCNI")]
    Dcni,
    /// Unclassified Controlled Nuclear Information - Energy. Relating to certain design and security
    /// information concerning nuclear facilities, materials, and weapons, specific to the Department of
    /// Energy.
    #[serde(rename = "UCNI")]
    Ucni,
    /// Victim. Information requiring protection of the name or other details that may identify one who
    /// was the victim of a crime.
    #[serde(rename = "LVIC")]
    Lvic,
    /// Victims of Human Trafficking. Related to identifiable information of persons who have been
    /// victims of human trafficking and their family members.
    #[serde(rename = "IVIC")]
    Ivic,
    /// Visas. Related to applications or permits to enter the United States.
    #[serde(rename = "VISA")]
    Visa,
    /// Water Assessments. Vulnerability Assessments on the risks and security of public drinking water
    /// systems, to include, but not be limited to, a review of pipes and constructed conveyances,
    /// physical barriers, water collection, pretreatment, treatment, storage and distribution
    /// facilities, electronic, computer or other automated systems which are utilized by the public
    /// water system, the use, storage, or handling of various chemicals, and the operation and
    /// maintenance of such system.
    #[serde(rename = "WATER")]
    Water,
    /// Whistleblower Identity. Identity of anyone providing information relating to a legal violation
    /// or illicit or unsafe activity, including information provided by a whistleblower which could
    /// reasonably be expected to reveal the identity of a whistleblower.
    #[serde(rename = "WHSTL")]
    Whstl,
    /// Witness Protection. Information related to the secretive details associated with one who has
    /// testified or may testify under circumstances that require secrecy of that individual and details
    /// pertaining to that person.
    #[serde(rename = "WIT")]
    Wit,
}

/// (U) All currently valid CUI Specified markings from the National Archives. This enum is used by
/// CUI_Specified. PERMISSIBLE VALUES The permissible values for this simple type are defined in the
/// Controlled Value Enumeration: CVEnumISMCUISpecified.xml
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CuiSpecifiedEnum {
    /// Accident Investigation. Related to information obtained during the course of an accident or
    /// incident investigation. Including but not limited to information related to wreckage, records,
    /// mail, or cargo.
    #[serde(rename = "AIV")]
    Aiv,
    /// Administrative Proceedings. Adjudication of agency-related matters including, but not limited
    /// to, dispute resolution, settlements, and issuances of orders.
    #[serde(rename = "ADPO")]
    Adpo,
    /// Ammonium Nitrate. Related to registration information of those who own and operate ammonium
    /// nitrate facilities, purchasers of ammonium nitrate, and the regulation of sales and transfers of
    /// ammonium nitrate.
    #[serde(rename = "CRITAN")]
    Critan,
    /// Archaelogical Resources. Related to information about the nature and location of any
    /// archaeological resource for which the excavation or removal requires a permit or other
    /// permission.
    #[serde(rename = "ARCHR")]
    Archr,
    /// Bank Secrecy. Information that is provided to the government pursuant to the Bank Secrecy Act,
    /// including but not limited to, suspicious activity reports (SAR), currency transaction reports
    /// (CTR), reports of international transportation of currency or monetary instruments (CMIR),
    /// reports of cash payment over $10,000 received in trade or business, and reports of foreign bank
    /// and financial accounts (FBAR). Reports filed under the Bank Secrecy Act (BSA), codified in
    /// relevant part at 31 U.S.C. § 5311 et seq, are specifically exempt from disclosure under the
    /// Freedom of Information Act, codified at 5 U.S.C. § 552, and also may not be disclosed under any
    /// State, local, tribal, or territorial "freedom of information," "open government," or similar
    /// law. See 31 U.S.C. § 5319; 5 U.S.C. § 552(b)(3). These reports (BSA Reports), are maintained in
    /// a system of records containing information compiled for law enforcement investigative purposes
    /// that has been exempted from the access provisions of the Privacy Act in accordance with 5 U.S.C.
    /// §§ 552a(j)(2) and (k)(2). BSA Reports may only be re-disseminated in strict accordance with
    /// guidelines established by the Financial Crimes Enforcement Network (FinCEN), the Treasury bureau
    /// that administers the BSA. Suspicious Activity Reports, one of the types of required reports
    /// filed under the BSA, are required to be kept confidential in accordance with 31 U.S.C. §
    /// 5318(g)(2) and implementing regulations. To the extent information falling under the purview of
    /// the BSA is collected, accessed, or used for any Federal tax administration purpose, it is also
    /// subject to the confidentiality provisions of the Internal Revenue Code, codified at 26 U.S.C. §
    /// 6103.
    #[serde(rename = "FSEC")]
    Fsec,
    /// Budget. Related to information concerning the federal budget, including authorizations and
    /// estimates of income and expenditures.
    #[serde(rename = "BUDG")]
    Budg,
    /// Campaign Funds. Related to information obtained in connection to an investigation into campaign
    /// finance and disclosure laws. Usage may include but is not limited to notification or
    /// investigation pertaining to financial support of a candidate for election.
    #[serde(rename = "FUND")]
    Fund,
    /// Chemical-terrorism Vulnerability Information. In accordance with Section 550(c) of the
    /// Department of Homeland Security Appropriations Act of 2007, the following information, whether
    /// transmitted verbally, electronically, or in written form, shall constitute CVI, see (1) - (9).
    #[serde(rename = "CVI")]
    Cvi,
    /// Child Pornography. From 18 USC 2256(8) "child pornography" means any visual depiction, including
    /// any photograph, film, video, picture, or computer or computer-generated image or picture,
    /// whether made or produced by electronic, mechanical, or other means, of sexually explicit
    /// conduct, where— (A) the production of such visual depiction involves the use of a minor engaging
    /// in sexually explicit conduct; (B) such visual depiction is a digital image, computer image, or
    /// computer-generated image that is, or is indistinguishable from, that of a minor engaging in
    /// sexually explicit conduct; or (C) such visual depiction has been created, adapted, or modified
    /// to appear that an identifiable minor is engaging in sexually explicit conduct
    #[serde(rename = "CHLD")]
    Chld,
    /// Consumer Complaint Information. Related to information concerning consumer complaints or
    /// inquiries concerning financial institutions or consumer financial products and services, and
    /// responses to them.
    #[serde(rename = "CCI")]
    Cci,
    /// Contract Use. Stipulations for a contractor to meet before material may be used in performance
    /// of certain contracts.
    #[serde(rename = "CONTRACT")]
    Contract,
    /// Controlled Substances. Information obtained by the Drug Enforcement Administration (DEA) or in
    /// DEA investigative reports related to controlled substances.
    #[serde(rename = "SUB")]
    Sub,
    /// Controlled Technical Information. Controlled Technical Information means technical information
    /// with military or space application that is subject to controls on the access, use, reproduction,
    /// modification, performance, display, release, disclosure, or dissemination. Controlled technical
    /// information is to be marked with one of the distribution statements B through F, in accordance
    /// with Department of Defense Instruction 5230.24, "Distribution Statements of Technical
    /// Documents." The term does not include information that is lawfully publicly available without
    /// restrictions. "Technical Information" means technical data or computer software, as those terms
    /// are defined in Defense Federal Acquisition Regulation Supplement clause 252.227-7013, "Rights in
    /// Technical Data - Noncommercial Items" (48 CFR 252.227-7013). Examples of technical information
    /// include research and engineering data, engineering drawings, and associated lists,
    /// specifications, standards, process sheets, manuals, technical reports, technical orders,
    /// catalog-item identifications, data sets, studies and analyses and related information, and
    /// computer software executable code and source code.
    #[serde(rename = "CTI")]
    Cti,
    /// Criminal History Records Information. Related to information collected by criminal justice
    /// agencies on individuals consisting of identifiable descriptions and notations of arrests,
    /// detentions, indictments, informations, or other formal criminal charges, and any disposition
    /// arising therefrom, including acquittal, sentencing, correctional supervision, and release.
    #[serde(rename = "CHRI")]
    Chri,
    /// Critical Energy Infrastructure Information. Critical energy infrastructure information means
    /// specific engineering, vulnerability, or detailed design information about proposed or existing
    /// critical infrastructure that: (i) Relates details about the production, generation,
    /// transportation, transmission, or distribution of energy; (ii) Could be useful to a person in
    /// planning an attack on critical infrastructure;... and (iii) Does not simply give the general
    /// location of the critical infrastructure.
    #[serde(rename = "CEII")]
    Ceii,
    /// DNA. Related to hereditary material in humans that is used for law enforcement purposes.
    #[serde(rename = "LDNA")]
    Ldna,
    /// Export Controlled. Unclassified information concerning certain items, commodities, technology,
    /// software, or other information whose export could reasonably be expected to adversely affect the
    /// United States national security and nonproliferation objectives. To include dual use items;
    /// items identified in export administration regulations, international traffic in arms regulations
    /// and the munitions list; license applications; and sensitive nuclear technology information.
    #[serde(rename = "EXPT")]
    Expt,
    /// Federal Grand Jury. Material obtained pursuant to a federal grand jury subpoena, which includes
    /// (1) any reference to a specific sitting grand jury; (2) any documentation or data obtained by a
    /// grand jury subpoena if disclosure of such material tends to reveal what transpired before or at
    /// the direction of the federal grand jury; (3) documentation prepared specifically for the federal
    /// grand jury; and (4) transcripts or other recordings of testimony presented to the federal grand
    /// jury.
    #[serde(rename = "JURY")]
    Jury,
    /// Federal Taxpayer Information. Related to returns and return information which are submitted,
    /// gathered or generated in conjunction with taxpayers' responsibilities to comply with federal tax
    /// provisions in the United States Code. "Returns" includes information that is provided to the
    /// government pursuant to Title 26, including tax or information returns, declarations of estimated
    /// tax or claims for refund. "Return information" includes a taxpayer's identity, the nature,
    /// source or amount of income or any information received by, recorded by, prepared by or furnished
    /// to Internal Revenue Service relevant to the determination of tax liability including whether the
    /// taxpayer is the subject of investigation. This protection extends to such items as medical,
    /// financial and other personal information submitted to the IRS by taxpayers. Standards (typically
    /// tolerances, audit criteria and law enforcement techniques) related to the selection of returns
    /// for examination should only be disclosed to the extent their disclosure would not impair
    /// assessment, collection or enforcement under the internal revenue laws. Tax data originating with
    /// the IRS generally retains its confidential status even when it resides with agencies other than
    /// the IRS.
    #[serde(rename = "TAX")]
    Tax,
    /// Foreign Intelligence Surveillance Act. Related to unclassified and declassified information that
    /// is collected from unconsenting individuals under the authority of the Foreign Intelligence
    /// Surveillance Act (FISA).
    #[serde(rename = "FISA")]
    Fisa,
    /// Foreign Intelligence Surveillance Act Business Records. Related to books, records, papers,
    /// documents, and other items produced for an investigation to obtain foreign intelligence
    /// information.
    #[serde(rename = "FISAB")]
    Fisab,
    /// General Financial Information. Related to the duties, transactions, or otherwise falling under
    /// the purview of financial institutions or United States Government fiscal functions. Uses may
    /// include, but are not limited to, customer information held by a financial institution.
    #[serde(rename = "FNC")]
    Fnc,
    /// General Intelligence. Related to intelligence activities, sources, or methods.
    #[serde(rename = "INTEL")]
    Intel,
    /// General Nuclear. Related to protection of information concerning nuclear reactors, materials, or
    /// security.
    #[serde(rename = "NUC")]
    Nuc,
    /// General Privacy. Refers to personal information, or, in some cases, "personally identifiable
    /// information," as defined in OMB M-17-12, or "means of identification" as defined in 18 USC
    /// 1028(d)(7).
    #[serde(rename = "PRVCY")]
    Prvcy,
    /// General Procurement and Acquisition. Material and information relating to, or associated with,
    /// the acquisition and procurement of goods and services, including but not limited to, cost or
    /// pricing data, contract information, indirect costs and direct labor rates.
    #[serde(rename = "PROCURE")]
    Procure,
    /// General Proprietary Business Information. Material and information relating to, or associated
    /// with, a company's products, business, or activities, including but not limited to financial
    /// information; data or statements; trade secrets; product research and development; existing and
    /// future product designs and performance specifications.
    #[serde(rename = "PROPIN")]
    Propin,
    /// Genetic Information. The term "genetic information" means, with respect to any individual,
    /// information about-- (i) such individual's genetic tests, (ii) the genetic tests of family
    /// members of such individual, and (iii) the manifestation of a disease or disorder in family
    /// members of such individual.
    #[serde(rename = "GENETIC")]
    Genetic,
    /// Geodetic Product Information. Related to imagery, imagery intelligence, or geospatial
    /// information.
    #[serde(rename = "GEO")]
    Geo,
    /// Health Information. As per 42 USC 1320d(4), "health information" means any information, whether
    /// oral or recorded in any form or medium, that (A) is created or received by a health care
    /// provider, health plan, public health authority, employer, life insurer, school or university, or
    /// health care clearinghouse; and (B) relates to the past, present, or future physical or mental
    /// health or condition of an individual, the provision of health care to an individual, or the
    /// past, present, or future payment for the provision of health care to an individual.
    #[serde(rename = "HLTH")]
    Hlth,
    /// Historic Properties. Related to the location, character, or ownership of historic property.
    #[serde(rename = "HISTP")]
    Histp,
    /// Informant. Related to the identity of a human source.
    #[serde(rename = "INF")]
    Inf,
    /// Inspector General Protected. Related to the identity of a person making a report to the
    /// Inspector General of any Executive agency.
    #[serde(rename = "PRIIG")]
    Priig,
    /// Intelligence Financial Records. Related to financial records obtained for intelligence or
    /// counterintelligence activity, investigation, or analysis.
    #[serde(rename = "IFNC")]
    Ifnc,
    /// Internal Data. Refers to a category of information that is not intended to be disseminated
    /// beyond CIA channels that involves intelligence activities, sources, or methods. This information
    /// may also relate to the CIA's organization, functions, names, official titles, salaries, or
    /// numbers of personnel.
    #[serde(rename = "ID")]
    Id,
    /// International Agreement Information. Information provided by, otherwise made available by, or
    /// produced in cooperation with, a foreign government or international organization that requires
    /// protection pursuant to an existing treaty, agreement, bilateral exchange or other obligation
    /// under the requirements stipulated in 10 USC 130c(b), when not subject to classification under
    /// Executive Order 13526. Title 10 USC 130c(b) may exempt this class of foreign government
    /// information from the safeguard provisions otherwise required by Executive Order 13526. Per Title
    /// 10 USC 130c(h) the following national security officials are the only ones defined by statute as
    /// able to determine such information requires control: (A) The Secretary of Defense, with respect
    /// to information of concern to the Department of Defense. (B) The Secretary of Homeland Security,
    /// with respect to information of concern to the Coast Guard, as determined by the Secretary, but
    /// only while the Coast Guard is not operating as a service in the Navy. (C) The Secretary of
    /// Energy, with respect to information concerning the national security programs of the Department
    /// of Energy, as determined by the Secretary.
    #[serde(rename = "INTL")]
    Intl,
    /// Investigation. Related to information obtained during the course of a law enforcement
    /// investigation or action, civil or criminal.
    #[serde(rename = "INV")]
    Inv,
    /// Law Enforcement Financial Records. Related to financial records obtained for law enforcement
    /// purposes.
    #[serde(rename = "LFNC")]
    Lfnc,
    /// National Park System Resources. Related to information concerning the nature and specific
    /// location of a National Park System resource that is endangered, threatened, rare, or
    /// commercially valuable, of mineral or paleontological objects within System units, or of objects
    /// of cultural patrimony within System units.
    #[serde(rename = "NPSR")]
    Npsr,
    /// Naval Nuclear Propulsion Information. Related to the safety of reactors and associated naval
    /// nuclear propulsion plants, and control of radiation and radioactivity associated with naval
    /// nuclear propulsion activities, including prescribing and enforcing standards and regulations for
    /// these areas as they affect the environment and the safety and health of workers, operators, and
    /// the general public.
    #[serde(rename = "NNPI")]
    Nnpi,
    /// Nuclear Security-Related Information. Related to information that could be useful, or could
    /// reasonably be expected to be useful, to a terrorist in a potential attack that does not qualify
    /// as Safeguards or classified information, including the exact location and quantities of
    /// radioactive material, certain detailed design drawings, information on nearby facilities,
    /// emergency planning information, and certain assessments of vulnerability and safety analyses.
    #[serde(rename = "SRI")]
    Sri,
    /// Personnel Records. Related to the employees of federal agencies.
    #[serde(rename = "PERS")]
    Pers,
    /// Proprietary Manufacturer. Relating to the production of a consumer product to include that of a
    /// private labeler.
    #[serde(rename = "MFC")]
    Mfc,
    /// Protected Critical Infrastructure Information. As defined by 6 USC 131-134, and 6 CFR 29, PCII
    /// relates to threats, vulnerabilities, or operational experience related to the national
    /// infrastructure. PCII offers protection to private sector infrastructure information voluntarily
    /// shared with government entities for purposes of homeland security.
    #[serde(rename = "PCII")]
    Pcii,
    /// Protective Order. Stipulation that certain information that would normally fall under discovery
    /// rules will not be disclosed for specifically stated reason.
    #[serde(rename = "LPROT")]
    Lprot,
    /// Safeguards Information. Pursuant to 42 USC 2011, et seq., and as defined in 10 CFR 73.2, SGI
    /// relates to security related information concerning the physical protection of source, byproduct
    /// or special nuclear material and the detailed security measures for facilities and information
    /// contained within security plans.
    #[serde(rename = "SGI")]
    Sgi,
    /// Sensitive Security Information. As defined in 49 C.F.R. Part 15.5, Sensitive Security
    /// Information is information obtained or developed in the conduct of security activities,
    /// including research and development, the disclosure of which DOT has determined would constitute
    /// an unwarranted invasion of privacy, reveal trade secrets or privileged or confidential
    /// information, or be detrimental to transportation safety. As defined in 49 C.F.R. Part 1520.5,
    /// Sensitive Security Information is information obtained or developed in the conduct of security
    /// activities, including research and development, the disclosure of which DHS/TSA has determined
    /// would, among other things, be detrimental to the security of transportation.
    #[serde(rename = "SSI")]
    Ssi,
    /// Source Selection. Per FAR 2.101: any of the following information that is prepared for use by an
    /// agency for the purpose of evaluating a bid or proposal to enter into an agency procurement
    /// contract, if that information has not been previously made available to the public or disclosed
    /// publicly: (Items 1-10).
    #[serde(rename = "SSEL")]
    Ssel,
    /// Statistical Information. Refers to information collected by a Federal statistical agency, unit,
    /// or program for statistical purposes or used for statistical activities; under law, regulation,
    /// or Government-wide policy such 'Statistical' CUI requires: (1) protection from unauthorized
    /// disclosure; (2) special handling safeguards; and/or (3) prescribed limits on access or
    /// dissemination.
    #[serde(rename = "STAT")]
    Stat,
    /// Student Records. As per 20 USC 1232g, the Family Educational Rights and Privacy Act of 1974, an
    /// education record which is comprised of those records which are directly related to a student.
    #[serde(rename = "STUD")]
    Stud,
    /// Toxic Substances. Health, safety, and exposure information related to chemical substances,
    /// chemical mixtures, and articles as defined under the Toxic Substances Control Act (TSCA).
    #[serde(rename = "TSCA")]
    Tsca,
    /// Unclassified Controlled Nuclear Information - Defense. Relating to Department of Defense special
    /// nuclear material (SNM), equipment, and facilities, as defined by 32 CFR 223.
    #[serde(rename = "DCNI")]
    Dcni,
    /// Unclassified Controlled Nuclear Information - Energy. Relating to certain design and security
    /// information concerning nuclear facilities, materials, and weapons, specific to the Department of
    /// Energy.
    #[serde(rename = "UCNI")]
    Ucni,
    /// US Census. Related to information gathered by the Bureau of the Census during the process of
    /// collecting, compiling, evaluating, analyzing of demographic, economic, and social data
    /// pertaining at a specified time to any or all persons in the United States and dissemination is
    /// limited to those with special sworn status, who may only use the data for statistical purposes
    /// and only for those statistical purposes for which the data was supplied.
    #[serde(rename = "CENS")]
    Cens,
    /// Whistleblower Identity. Identity of anyone providing information relating to a legal violation
    /// or illicit or unsafe activity, including information provided by a whistleblower which could
    /// reasonably be expected to reveal the identity of a whistleblower.
    #[serde(rename = "WHSTL")]
    Whstl,
    /// Witness Protection. Information related to the secretive details associated with one who has
    /// testified or may testify under circumstances that require secrecy of that individual and details
    /// pertaining to that person.
    #[serde(rename = "WIT")]
    Wit,
    /// Written Determinations. Rulings, determination letters, technical advice memoranda or Chief
    /// Counsel Advice, as those terms are defined in Treasury Regulation 301.6110-2, which are made
    /// available for public inspection subject to the withholding of certain types of data as
    /// enumerated in Treasury Regulation 301-6110.
    #[serde(rename = "WDT")]
    Wdt,
}

/// The reason why an action (Task, Command, Activity, etc.) was rejected, interrupted, unallocated
/// or failed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CannotComplyEnum {
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was aborted due to constraints and/or priorities of the mission or
    /// system. This is different from an action that was canceled, rejected, or failed.
    #[serde(rename = "ABORTED")]
    Aborted,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because the store reports conditions which indicate that their
    /// Inertial Navigation System (INS) platform quality is poor and the platform should perform a
    /// maneuver to allow system to align itself.
    #[serde(rename = "ALIGNMENT_MANEUVER")]
    AlignmentManeuver,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was canceled, generally by the originating source. This is a
    /// generalization of the previous USER_CANCELED indication.
    #[serde(rename = "CANCELED")]
    Canceled,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because the associated Capability is faulted.
    #[serde(rename = "CAPABILITY_FAULT")]
    CapabilityFault,
    /// Indicates that Resource Manager cannot allocate resources to the action (Task,
    /// \[Capability\]Command, \[SupportingCapability\]Command, \[Capability\]Activity, etc.) such that
    /// the PerformanceConstraints in PerformanceMetricSetType are met.
    #[serde(rename = "CAPABILITY_PERFORMANCE")]
    CapabilityPerformance,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because the precedence of its associated Capability within the
    /// associated Subsystem wasn't high enough for the action to proceed.
    #[serde(rename = "CAPABILITY_PRECEDENCE")]
    CapabilityPrecedence,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because of a range related limitation of the associated Capability.
    /// This is a generalization of the former INSUFFICIENT_SENSOR_RANGE indication.
    #[serde(rename = "CAPABILITY_RANGE")]
    CapabilityRange,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because the associated Capability wasn't available at the needed
    /// time.
    #[serde(rename = "CAPABILITY_UNAVAILABLE")]
    CapabilityUnavailable,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because the number of attempts specified in the command was reached
    /// before the action achieved the desired result.
    #[serde(rename = "CONSTRAINT_ATTEMPTS")]
    ConstraintAttempts,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because the classification of the action, actor, capability or
    /// associated elements is inconsistent.
    #[serde(rename = "CONSTRAINT_CLASSIFICATION")]
    ConstraintClassification,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because of insufficient endurance. For example, fuel or battery
    /// limitations. This is a generalization of the former INSUFFICIENT_FUEL_QUANTITY indication.
    #[serde(rename = "CONSTRAINT_ENDURANCE")]
    ConstraintEndurance,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because it exceeds field of regard (FOR) or field of view (FOV)
    /// limitations.
    #[serde(rename = "CONSTRAINT_FOR_FOV_LIMIT")]
    ConstraintForFovLimit,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because of a gating constraint on the command. For example, the
    /// target of the action didn't meet velocity, range, power, etc. gates/filters in the command.
    #[serde(rename = "CONSTRAINT_GATING")]
    ConstraintGating,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because maneuvering exceeds maneuver limits of the action.
    #[serde(rename = "CONSTRAINT_MANEUVER_LIMIT")]
    ConstraintManeuverLimit,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because of line of sight limitations due to terrain occlusion.
    #[serde(rename = "CONSTRAINT_OCCLUSION")]
    ConstraintOcclusion,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because of a constraint imposed by an "Op" message (OpZone, OpLine,
    /// OpRouting, etc.). For example, a Strike Task in a NO_FIRE OpZone. This is a generalization of
    /// the previous NO_FIRE or NO_IMAGE indications.
    #[serde(rename = "CONSTRAINT_OP")]
    ConstraintOp,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily due to operational security.
    #[serde(rename = "CONSTRAINT_OPSEC")]
    ConstraintOpsec,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because an unspecified constraint was violated.
    #[serde(rename = "CONSTRAINT_OTHER")]
    ConstraintOther,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because of a conflict between the needed/commanded RF parameters and
    /// the RF spectrum available. For example, the commanded RF band isn't supported by the loaded MDF
    /// of a Capability or there is an active RF_Profile that doesn't allow transmission in the
    /// commanded band. This is a generalization of the former INVALID_RF_BAND, RF_SPECTRUM_LIMITED,
    /// RF_UNAVAILABLE and RF_PROFILE_RESTRICTION.
    #[serde(rename = "CONSTRAINT_RF")]
    ConstraintRf,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because it isn't achievable given the currently planned route.
    /// Alternatively, a route to achieve the action couldn't be generated.
    #[serde(rename = "CONSTRAINT_ROUTE")]
    ConstraintRoute,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because of a safety related constraint.
    #[serde(rename = "CONSTRAINT_SAFETY")]
    ConstraintSafety,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because it was constrained for allocation to a specific System that
    /// cannot complete it. This is a generalization of the previous INELIGIBLE_VEHICLE indication.
    #[serde(rename = "CONSTRAINT_SYSTEM")]
    ConstraintSystem,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because of command constraints on the angle between the actor and
    /// the target of the action.
    #[serde(rename = "CONSTRAINT_TARGET_ANGLE")]
    ConstraintTargetAngle,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because it has temporal requirements that can't be achieved or were
    /// reached before the action achieved the desired result.
    #[serde(rename = "CONSTRAINT_TIME")]
    ConstraintTime,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because it has an "all or nothing" association with another action
    /// that wasn't allocated, unallocated, failed, etc. This is a generalization of the former
    /// ALL_OR_NOTHING_TASK_CONSTRAINT.
    #[serde(rename = "DEPENDENCY_ALL_OR_NOTHING")]
    DependencyAllOrNothing,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because it has an "either or" association with another action that
    /// wasn't allocated, unallocated, failed, etc. This is a generalization of the former
    /// EITHER_OR_TASK_CONSTRAINT.
    #[serde(rename = "DEPENDENCY_EITHER_OR")]
    DependencyEitherOr,
    /// Indicates the action (Task, \[Capability\]Command,\[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because it has a constraint for a multiple-system allocation to one
    /// or more actions that wasn't allocated, unallocated, failed, etc.
    #[serde(rename = "DEPENDENCY_MULTIPLE_SYSTEM")]
    DependencyMultipleSystem,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because a required predecessor action wasn't allocated, unallocated,
    /// failed, etc. This is a generalization of the former UNASSIGNABLE_DEPENDENT_TASK.
    #[serde(rename = "DEPENDENCY_PREDECESSOR")]
    DependencyPredecessor,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because it has a constraint for a same-system allocation to one or
    /// more actions that wasn't allocated, unallocated, failed, etc.
    #[serde(rename = "DEPENDENCY_SAME_SYSTEM")]
    DependencySameSystem,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because the number of DMPIs designated has exceeded the maximum
    /// threshold.
    #[serde(rename = "DMPI_OVERDESIGNATION")]
    DmpiOverdesignation,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because the number of DMPIs designated has not met the minimum
    /// threshold.
    #[serde(rename = "DMPI_UNDERDESIGNATION")]
    DmpiUnderdesignation,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because the source of the action isn't eligible and/or hasn't been
    /// granted permission to control (mission control, primary capability control, secondary capability
    /// control, automated capability management) the associated System or Capability. This is a
    /// generalization of the former SYSTEM_NOT_IN_CONTROL and SERVICE_NOT_IN_CONTROL.
    #[serde(rename = "INELIGIBLE_CONTROL_SOURCE")]
    IneligibleControlSource,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because a route to achieve the action couldn't be generated.
    #[serde(rename = "INFEASIBLE_ROUTE")]
    InfeasibleRoute,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because the initialization conditions were not met.
    #[serde(rename = "INIT_CRITERIA_NOT_MET")]
    InitCriteriaNotMet,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because of another unspecified input problem.
    #[serde(rename = "INPUT_OTHER")]
    InputOther,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because of insufficient accuracy information (e.g., track accuracy).
    #[serde(rename = "INSUFFICIENT_ACCURACY")]
    InsufficientAccuracy,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because of insufficient resources (e.g. signal processors, memory or
    /// other items generally subordinate to Subsystems and Capabilities). In this condition, the
    /// associated Capability, Subsystem and System should be otherwise able to perform the action.
    #[serde(rename = "INSUFFICIENT_RESOURCES")]
    InsufficientResources,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because a parameter in the action or other associated input was
    /// invalid.
    #[serde(rename = "INVALID_INPUT_PARAMETER")]
    InvalidInputParameter,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because of a system or platform's inability to execute tasking,
    /// tracking, or other capabilities due to the presence of jamming.
    #[serde(rename = "JAMMING_DETECTED")]
    JammingDetected,
    /// Indicates the action to activate an MDF (\[Capability\]SettingsCommand was rejected because the
    /// commanded MDF_ID and/or SubCategory FileID is invalid or represents a file set that is not
    /// compliant with the Capability.
    #[serde(rename = "MDF_ACTIVATION_ERROR")]
    MdfActivationError,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily due to a general mission event (flight contingency, mission
    /// contingency, allocation change, etc.). This is generalization of the previous MISSION_EVENT
    /// indication.
    #[serde(rename = "MISSION_EVENT")]
    MissionEvent,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because of multiple sibling enumeration reasons but a specific
    /// single reason isn't known.
    #[serde(rename = "MULTIPLE")]
    Multiple,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because the operational intent was not met.
    #[serde(rename = "OPERATIONAL_INTENT_NOT_MET")]
    OperationalIntentNotMet,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because of another reason which hasn't yet been incorporated into
    /// the enumeration.
    #[serde(rename = "OTHER")]
    Other,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because its ranking wasn't high enough to gain or maintain access to
    /// the required capability.
    #[serde(rename = "RANKING")]
    Ranking,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because an associated System, Subsystem or other item is
    /// reconstituting.
    #[serde(rename = "RECONSTITUTING")]
    Reconstituting,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because of current/future state or settings of the corresponding
    /// Capability or Subsystem.
    #[serde(rename = "STATE_OR_SETTINGS")]
    StateOrSettings,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because a change in the state or settings of the corresponding
    /// Capability or Subsystem.
    #[serde(rename = "STATE_OR_SETTINGS_CHANGE")]
    StateOrSettingsChange,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because the associated Subsystem is faulted.
    #[serde(rename = "SUBSYSTEM_FAULT")]
    SubsystemFault,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because the associated Subsystem wasn't available at the needed
    /// time.
    #[serde(rename = "SUBSYSTEM_UNAVAILABLE")]
    SubsystemUnavailable,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because of degraded/failed/mismatched cryptographic services.
    #[serde(rename = "SYSTEM_CANNOT_PROCESS_CRYPTO_FAILURE")]
    SystemCannotProcessCryptoFailure,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because the intended network is undefined or unavailable.
    #[serde(rename = "SYSTEM_CANNOT_PROCESS_NETWORK_NOT_DEFINED")]
    SystemCannotProcessNetworkNotDefined,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because the intended destination of the action did not respond after
    /// receipt.
    #[serde(rename = "SYSTEM_CANNOT_PROCESS_NO_RESPONSE_AFTER_FORWARDING")]
    SystemCannotProcessNoResponseAfterForwarding,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because the system required to take action could not process the
    /// request as it was malformed (e.g., unexpected integer in a constraint lookup).
    #[serde(rename = "SYSTEM_CANNOT_PROCESS_REQUEST_MALFORMED")]
    SystemCannotProcessRequestMalformed,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because the requested action cannot be supported.
    #[serde(rename = "SYSTEM_CANNOT_PROCESS_REQUEST_NOT_SUPPORTED")]
    SystemCannotProcessRequestNotSupported,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because the action results in conflict with another System's route,
    /// sensor, weapon or other plans/actions.
    #[serde(rename = "SYSTEM_CONFLICT")]
    SystemConflict,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because the associated System is faulted.
    #[serde(rename = "SYSTEM_FAULT")]
    SystemFault,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because the associated System wasn't available at the needed time.
    #[serde(rename = "SYSTEM_UNAVAILABLE")]
    SystemUnavailable,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because of an unknown reason.
    #[serde(rename = "UNKNOWN")]
    Unknown,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because the object corresponding to an ID in the command could not
    /// be found.
    #[serde(rename = "UNKNOWN_ID")]
    UnknownId,
    /// Indicates the action (Task, \[Capability\]Command, \[SupportingCapability\]Command,
    /// \[Capability\]Activity, etc.) was rejected, interrupted, unallocated, failed or generally can't
    /// be completed satisfactorily because of weather effects.
    #[serde(rename = "WEATHER")]
    Weather,
}

/// (U) All currently valid classification marks PERMISSIBLE VALUES The permissible values for this
/// simple type are defined in the Controlled Value Enumeration: CVEnumISMClassificationAll.xml
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClassificationEnum {
    /// RESTRICTED
    #[serde(rename = "R")]
    R,
    /// CONFIDENTIAL
    #[serde(rename = "C")]
    C,
    /// SECRET
    #[serde(rename = "S")]
    S,
    /// TOP SECRET
    #[serde(rename = "TS")]
    Ts,
    /// UNCLASSIFIED
    #[serde(rename = "U")]
    U,
}

/// Indicates the state of the associated command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandProcessingStateEnum {
    /// Indicates a \[NEW\] Command has been received and the Subsystem or Service is determining if it
    /// can perform the Command. An \[UPDATE\] Command can be received in this state which will modify
    /// the command parameters.
    #[serde(rename = "RECEIVED")]
    Received,
    /// Indicates the Subsystem or Service has accepted the Command and will subsequently status it
    /// through \[Capability\]Activity or other another status message. This is a terminal state and the
    /// Subsystem or Service will ignore all subsequent updates to the Command, including CANCEL.
    #[serde(rename = "ACCEPTED")]
    Accepted,
    /// Indicates the Subsystem or Service has determined that it will not perform the Command. This is
    /// a terminal state and the Subsystem or Service will ignore all subsequent updates to the Command,
    /// including CANCEL.
    #[serde(rename = "REJECTED")]
    Rejected,
    /// Indicates a \[CANCEL\] Command was been received while the Subsystem or Service could honor it
    /// and the Command was removed from consideration with no resulting Activity and/or functionality
    /// starting.
    #[serde(rename = "CANCELED")]
    Canceled,
}

/// Indicates whether an instance of a message is new, an update to an existing message/object or a
/// request/command to remove the message/object from the network.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandStateEnum {
    /// Indicates that the command message is new to the network.
    #[serde(rename = "NEW")]
    New,
    /// Indicates that this command message is an update to a command message previously placed on the
    /// network.
    #[serde(rename = "UPDATE")]
    Update,
    /// Indicates that the command should be canceled.
    #[serde(rename = "CANCEL")]
    Cancel,
}

/// UCI uses the W3C (www.w3.org) definition of date and time exactly as given in the specification
/// for xs:dateTime with a further restriction that only the "Zulu" time zone be used. xs:dateTime
/// is based on Coordinated Universal Time (UTC) and allows seconds to be specified with decimal
/// digits to arbitrary precision. See the W3C specification of xs:dateTime for further details.
pub type DateTimeType = String;

/// (U) All currently authorized authority block declass date/event exemptions. PERMISSIBLE VALUES
/// The permissible values for this simple type are defined in the Controlled Value Enumeration:
/// CVEnumISM25X.xml
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeclassExceptionEnum {
    /// When using a source document that contains portions of Restricted Data (RD) or Formerly
    /// Restricted Data (FRD) where the RD/FRD source document(s) do not have declassification
    /// instructions, the derivatively classified document shall not contain a declassification date or
    /// event on the Declassify On line. The following shall be annotated on the Declassify On line:
    /// "Not Applicable or (N/A) to RD/FRD portions" and "See source list for NSI portions" separated by
    /// a period. The source list must include the declassification instruction for each of the source
    /// documents classified under E.O. 13526 and shall not appear in the classification authority block
    #[serde(rename = "AEA")]
    Aea,
    /// Since NATO information is not to be declassified or downgraded without the prior consent of
    /// NATO, the "Declassify on" line of documents that commingle information classified by NATO and
    /// U.S. classified NSI, will read "N/A to NATO portions. See source list for NSI portions." The NSI
    /// source list will appear beneath the classification authority block in a manner that clearly
    /// identifies it as separate and distinct.
    #[serde(rename = "NATO")]
    Nato,
    /// Handles special case of BOTH NATO and AEA as a single exemption.
    #[serde(rename = "NATO_AEA")]
    NatoAea,
    /// Reveal the identity of a confidential human source, a human intelligence source, a relationship
    /// with an intelligence or security service of a foreign government or international organization,
    /// or a non-human intelligence source; or impair the effectiveness of an intelligence method
    /// currently in use, available for use, or under development.
    #[serde(rename = "25X1")]
    Value25X1,
    /// "25X1, EO 12951" (prescribed by the DNI for use on information described in E.O. 12951, Release
    /// of Imagery Acquired by Space-Based National Intelligence Reconnaissance Systems)
    #[serde(rename = "25X1_EO_12951")]
    Value25X1Eo12951,
    /// Reveal information that would assist in the development, production, or use of weapons of mass
    /// destruction.
    #[serde(rename = "25X2")]
    Value25X2,
    /// Reveal information that would impair U.S. cryptologic systems or activities.
    #[serde(rename = "25X3")]
    Value25X3,
    /// Reveal information that would impair the application of state-of-the-art technology within a
    /// U.S. weapon system.
    #[serde(rename = "25X4")]
    Value25X4,
    /// Reveal formally named or numbered U.S. military war plans that remain in effect, or reveal
    /// operational or tactical elements of prior plans that are contained in such active plans.
    #[serde(rename = "25X5")]
    Value25X5,
    /// Reveal information, including foreign government information, that would cause serious harm to
    /// relations between the United States and a foreign government, or to ongoing diplomatic
    /// activities of the United States.
    #[serde(rename = "25X6")]
    Value25X6,
    /// Reveal information that would impair the current ability of United States Government officials
    /// to protect the President, Vice President, and other protectees for whom protection services, in
    /// the interest of the national security, are authorized.
    #[serde(rename = "25X7")]
    Value25X7,
    /// Reveal information that would seriously impair current national security emergency preparedness
    /// plans or reveal current vulnerabilities of systems, installations, or infrastructures relating
    /// to the national security.
    #[serde(rename = "25X8")]
    Value25X8,
    /// Violate a statute, treaty, or international agreement that does not permit the automatic or
    /// unilateral declassification of information at 25 years.
    #[serde(rename = "25X9")]
    Value25X9,
    /// The ISCAP has authorized use of this code in the FBI's classification guidance (which results in
    /// a 75-year classification period) for any agency sourcing/reusing the information.
    #[serde(rename = "50X1")]
    Value50X1,
    /// When the information clearly and demonstrably could be expected to reveal the identity of a
    /// confidential human source or a human intelligence source.
    #[serde(rename = "50X1_HUM")]
    Value50X1Hum,
    /// Reveal information that would assist in the development, production, or use of weapons of mass
    /// destruction.
    #[serde(rename = "50X2")]
    Value50X2,
    /// When the information clearly and demonstrably could reveal key design concepts of weapons of
    /// mass destruction.
    #[serde(rename = "50X2_WMD")]
    Value50X2Wmd,
    /// Reveal information that would impair U.S. cryptologic systems or activities.
    #[serde(rename = "50X3")]
    Value50X3,
    /// Reveal information that would impair the application of state-of-the-art technology within a
    /// U.S. weapon system.
    #[serde(rename = "50X4")]
    Value50X4,
    /// Reveal formally named or numbered U.S. military war plans that remain in effect, or reveal
    /// operational or tactical elements of prior plans that are contained in such active plans.
    #[serde(rename = "50X5")]
    Value50X5,
    /// Reveal information, including foreign government information, that would cause serious harm to
    /// relations between the United States and a foreign government, or to ongoing diplomatic
    /// activities of the United States.
    #[serde(rename = "50X6")]
    Value50X6,
    /// Reveal information that would impair the current ability of United States Government officials
    /// to protect the President, Vice President, and other protectees for whom protection services, in
    /// the interest of the national security, are authorized.
    #[serde(rename = "50X7")]
    Value50X7,
    /// Reveal information that would seriously impair current national security emergency preparedness
    /// plans or reveal current vulnerabilities of systems, installations, or infrastructures relating
    /// to the national security.
    #[serde(rename = "50X8")]
    Value50X8,
    /// Violate a statute, treaty, or international agreement that does not permit the automatic or
    /// unilateral declassification of information at 25 years.
    #[serde(rename = "50X9")]
    Value50X9,
    /// Specific information that has been formally approved by the ISCAP as information that does not
    /// permit the automatic or unilateral declassification of information at 75 years.
    #[serde(rename = "75X")]
    Value75X,
}

/// (U) All currently valid Dissemination controls from the published register PERMISSIBLE VALUES
/// The permissible values for this simple type are defined in the Controlled Value Enumeration:
/// CVEnumISMDissem.xml
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisseminationControlsEnum {
    /// RISK SENSITIVE
    #[serde(rename = "RS")]
    Rs,
    /// FOR OFFICIAL USE ONLY
    #[serde(rename = "FOUO")]
    Fouo,
    /// ORIGINATOR CONTROLLED
    #[serde(rename = "OC")]
    Oc,
    /// ORIGINATOR CONTROLLED US GOVERNMENT
    #[serde(rename = "OC_USGOV")]
    OcUsgov,
    /// CONTROLLED IMAGERY
    #[serde(rename = "IMC")]
    Imc,
    /// NOT RELEASABLE TO FOREIGN NATIONALS
    #[serde(rename = "NF")]
    Nf,
    /// CAUTION-PROPRIETARY INFORMATION INVOLVED
    #[serde(rename = "PR")]
    Pr,
    /// AUTHORIZED FOR RELEASE TO
    #[serde(rename = "REL")]
    Rel,
    /// RELEASABLE BY INFORMATION DISCLOSURE OFFICIAL
    #[serde(rename = "RELIDO")]
    Relido,
    /// EYES ONLY
    #[serde(rename = "EYES")]
    Eyes,
    /// DEA SENSITIVE
    #[serde(rename = "DSEN")]
    Dsen,
    /// RAW FOREIGN INTELLIGENCE SURVEILLANCE ACT
    #[serde(rename = "RAWFISA")]
    Rawfisa,
    /// FOREIGN INTELLIGENCE SURVEILLANCE ACT
    #[serde(rename = "FISA")]
    Fisa,
    /// AUTHORIZED FOR DISPLAY BUT NOT RELEASE TO
    #[serde(rename = "DISPLAYONLY")]
    Displayonly,
    /// EXEMPT FROM ICD501 DISCOVERY
    #[serde(rename = "EXEMPT_FROM_ICD501_DISCOVERY")]
    ExemptFromIcd501Discovery,
    /// WAIVED
    #[serde(rename = "WAIVED")]
    Waived,
    /// Attorney-Client
    #[serde(rename = "AC")]
    Ac,
    /// Attorney-WP
    #[serde(rename = "AWP")]
    Awp,
    /// DL ONLY
    #[serde(rename = "DL_ONLY")]
    DlOnly,
    /// FED ONLY
    #[serde(rename = "FED_ONLY")]
    FedOnly,
    /// FEDCON
    #[serde(rename = "FEDCON")]
    Fedcon,
    /// NOCON
    #[serde(rename = "NOCON")]
    Nocon,
}

/// UCI uses the W3C (www.w3.org) definition of time duration exactly as given in the specification
/// for xs:duration. xs:duration is based on Coordinated Universal Time (UTC) and allows seconds to
/// be specified with decimal digits to arbitrary precision. See the W3C specification of
/// xs:duration for further details.
pub type DurationType = String;

/// Provides a list of possible entity sources.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntitySourceEnum {
    /// Indicates the Entity is an OB record (Order of Battle) message or other OB-oriented data source
    /// that has been promoted to an Entity.
    #[serde(rename = "ORDER_OF_BATTLE")]
    OrderOfBattle,
    /// Indicates the Entity originated from an IFF (Identification, Friend or Foe) data source such a
    /// radio.
    #[serde(rename = "IFF")]
    Iff,
    /// Indicates the Entity originated from an AIS (maritime Automatic Identification System) data
    /// source such as a radio.
    #[serde(rename = "AIS")]
    Ais,
    /// Indicates the Entity originated from a BFT (Blue Force Tracker) data source.
    #[serde(rename = "BFT")]
    Bft,
    /// Indicates the Entity is a MIL-STD-6018 IBS "track" message that has been republished as an
    /// Entity.
    #[serde(rename = "EXTERNAL_IBS")]
    ExternalIbs,
    /// Indicates the Entity is a MIL-STD-6016 "track" message that has been republished as an Entity.
    #[serde(rename = "EXTERNAL_LINK16")]
    ExternalLink16,
    /// Indicates the Entity is a MIL-STD-6016 electronic warfare product message that has been
    /// republished as an Entity.
    #[serde(rename = "EXTERNAL_LINK16_EW")]
    ExternalLink16Ew,
    /// Indicates the Entity is a republication or a "track" from another non-UCI data source.
    #[serde(rename = "EXTERNAL_OTHER")]
    ExternalOther,
    /// Indicates the Entity is an output of a fusion Service at the C2 node of a potentially tiered
    /// fusion system.
    #[serde(rename = "FUSION_C2")]
    FusionC2,
    /// Indicates the Entity is an output of a fusion Service at the space surveillance node of a
    /// potentially tiered fusion system.
    #[serde(rename = "FUSION_SPACE_SURVEILLANCE")]
    FusionSpaceSurveillance,
    /// Indicates the Entity is an output of a fusion Service at a vehicle control station or vehicle
    /// node of a potentially tiered fusion system.
    #[serde(rename = "FUSION_SYSTEM")]
    FusionSystem,
    /// Indicates the Entity is an output of a fusion Service at an unspecified type of node. This value
    /// is meant for temporary use while a more formal definition/name of the node, from a fusion
    /// perspective, is developed and incorporated into the standard.
    #[serde(rename = "FUSION_OTHER")]
    FusionOther,
    /// Indicates the Entity is derived from data from another UCI message. This is an indication that
    /// the data in the source message has been promoted to an Entity in order to enable Entity-oriented
    /// use-cases and interactions with the data. For example, an OpPoint or OperatorLocationOfInterest
    /// could be promoted to an Entity to allow Tasks and fusion against it.
    #[serde(rename = "INTERNALLY_DERIVED")]
    InternallyDerived,
    /// Indicates the Entity was manually created by a human operator.
    #[serde(rename = "OPERATOR")]
    Operator,
    /// Indicates the Entity originated from an active optical sensor track Capability.
    #[serde(rename = "CAPABILITY_ACTIVE_OPTICAL")]
    CapabilityActiveOptical,
    /// Indicates the Entity originated from a passive optical sensor Capability.
    #[serde(rename = "CAPABILITY_PASSIVE_OPTICAL")]
    CapabilityPassiveOptical,
    /// Indicates the Entity originated from a radar SAR sensor Capability.
    #[serde(rename = "CAPABILITY_SAR")]
    CapabilitySar,
    /// Indicates the Entity originated from a radar SMTI (Surface Moving Target Indicator, ground or
    /// maritime) sensor Capability.
    #[serde(rename = "CAPABILITY_SMTI")]
    CapabilitySmti,
    /// Indicates the Entity originated from a radar AMTI (Air Moving Target Indicator) sensor
    /// Capability.
    #[serde(rename = "CAPABILITY_AMTI")]
    CapabilityAmti,
    /// Indicates the Entity originated from a signals intelligence, specifically ESM (Electronic
    /// Support Measures), sensor Capability.
    #[serde(rename = "CAPABILITY_ESM")]
    CapabilityEsm,
    /// Indicates the Entity originated from a signals intelligence, specifically ELINT (Electronic
    /// Signals Intelligence), sensor Capability.
    #[serde(rename = "CAPABILITY_ELINT")]
    CapabilityElint,
    /// Indicates the Entity originated from a signals intelligence, specifically COMINT (Communications
    /// Intelligence), sensor Capability.
    #[serde(rename = "CAPABILITY_COMINT")]
    CapabilityComint,
    /// Indicates the Entity originated from an EA Capability. Detection and tracking of Entities that
    /// are electronically attacked (EA) is often done by an independent ESM (sibling enumerate
    /// CAPABILITY_ESM) or other "sensor" Capability. However, EA Capabilities are often implemented
    /// with a built-in ESM-like Capability to track a signal and optimize the EA response in real time;
    /// this enumerate is only intended for this case. This enumerate is not intended to represent an
    /// ESM Capability that operates independent of EA.
    #[serde(rename = "CAPABILITY_EA")]
    CapabilityEa,
    /// Indicates the Entity originated from an Orbital Surveillance capability.
    #[serde(rename = "CAPABILITY_ORBITAL_SURVEILLANCE")]
    CapabilityOrbitalSurveillance,
    /// Indicates the Entity originated from a non-specified sensor Capability. This value is meant for
    /// temporary use while a more formal definition/name of the Capability is developed and
    /// incorporated into the UCI standard.
    #[serde(rename = "CAPABILITY_OTHER")]
    CapabilityOther,
    /// Indicates the Entity originated from a Product such as a sensor image file.
    #[serde(rename = "PRODUCT_DERIVED")]
    ProductDerived,
    /// Indicates the Entity originated from an unknown, unspecified type of source. This value is meant
    /// for temporary use while a more formal definition/name of the source is developed and
    /// incorporated into the UCI standard.
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

/// An enumeration specifying the significance of the audit event. See enumerated type annotations
/// for further details.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventSeverityEnum {
    /// A security event has been detected and significant information or mechanisms have been
    /// compromised. The system may no longer be assumed to be operating correctly in support of the
    /// security policy. Immediate action is required.
    #[serde(rename = "CRITICAL")]
    Critical,
    /// A security event has been detected and less significant information or mechanisms have been
    /// compromised. Action is required within a given time.
    #[serde(rename = "ERROR")]
    Error,
    /// Normal operational messages that may be collected for reporting, measuring throughput, etc. No
    /// action is required.
    #[serde(rename = "INFORMATIONAL")]
    Informational,
    /// A warning indication that a security event may occur if action is not taken.
    #[serde(rename = "WARNING")]
    Warning,
}

/// CVEnumISMCATFGIOpen Values
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FgiSourceOpenEnum {
    /// Aruba
    #[serde(rename = "ABW")]
    Abw,
    /// Islamic Republic of Afghanistan
    #[serde(rename = "AFG")]
    Afg,
    /// Republic of Angola
    #[serde(rename = "AGO")]
    Ago,
    /// Anguilla
    #[serde(rename = "AIA")]
    Aia,
    /// Republic of Albania
    #[serde(rename = "ALB")]
    Alb,
    /// Principality of Andorra
    #[serde(rename = "AND")]
    And,
    /// United Arab Emirates
    #[serde(rename = "ARE")]
    Are,
    /// Argentine Republic
    #[serde(rename = "ARG")]
    Arg,
    /// Republic of Armenia
    #[serde(rename = "ARM")]
    Arm,
    /// Territory of American Samoa
    #[serde(rename = "ASM")]
    Asm,
    /// Antarctica
    #[serde(rename = "ATA")]
    Ata,
    /// French Southern and Antarctic Lands
    #[serde(rename = "ATF")]
    Atf,
    /// Antigua and Barbuda
    #[serde(rename = "ATG")]
    Atg,
    /// Commonwealth of Australia
    #[serde(rename = "AUS")]
    Aus,
    /// Republic of Austria
    #[serde(rename = "AUT")]
    Aut,
    /// Unknown
    #[serde(rename = "AX1")]
    Ax1,
    /// Guantanamo Bay Naval Base
    #[serde(rename = "AX2")]
    Ax2,
    /// Entity 6
    #[serde(rename = "AX3")]
    Ax3,
    /// Republic of Azerbaijan
    #[serde(rename = "AZE")]
    Aze,
    /// Republic of Burundi
    #[serde(rename = "BDI")]
    Bdi,
    /// Kingdom of Belgium
    #[serde(rename = "BEL")]
    Bel,
    /// Republic of Benin
    #[serde(rename = "BEN")]
    Ben,
    /// Bonaire, Sint Eustatius, and Saba
    #[serde(rename = "BES")]
    Bes,
    /// Burkina Faso
    #[serde(rename = "BFA")]
    Bfa,
    /// People's Republic of Bangladesh
    #[serde(rename = "BGD")]
    Bgd,
    /// Republic of Bulgaria
    #[serde(rename = "BGR")]
    Bgr,
    /// Kingdom of Bahrain
    #[serde(rename = "BHR")]
    Bhr,
    /// Commonwealth of The Bahamas
    #[serde(rename = "BHS")]
    Bhs,
    /// Bosnia and Herzegovina
    #[serde(rename = "BIH")]
    Bih,
    /// Saint Barthelemy
    #[serde(rename = "BLM")]
    Blm,
    /// Republic of Belarus
    #[serde(rename = "BLR")]
    Blr,
    /// Belize
    #[serde(rename = "BLZ")]
    Blz,
    /// Bermuda
    #[serde(rename = "BMU")]
    Bmu,
    /// Plurinational State of Bolivia
    #[serde(rename = "BOL")]
    Bol,
    /// Federative Republic of Brazil
    #[serde(rename = "BRA")]
    Bra,
    /// Barbados
    #[serde(rename = "BRB")]
    Brb,
    /// Brunei Darussalam
    #[serde(rename = "BRN")]
    Brn,
    /// Kingdom of Bhutan
    #[serde(rename = "BTN")]
    Btn,
    /// Bouvet Island
    #[serde(rename = "BVT")]
    Bvt,
    /// Republic of Botswana
    #[serde(rename = "BWA")]
    Bwa,
    /// Central African Republic
    #[serde(rename = "CAF")]
    Caf,
    /// Canada
    #[serde(rename = "CAN")]
    Can,
    /// Territory of Cocos (Keeling) Islands
    #[serde(rename = "CCK")]
    Cck,
    /// Swiss Confederation
    #[serde(rename = "CHE")]
    Che,
    /// Republic of Chile
    #[serde(rename = "CHL")]
    Chl,
    /// People's Republic of China
    #[serde(rename = "CHN")]
    Chn,
    /// Republic of Côte d'Ivoire
    #[serde(rename = "CIV")]
    Civ,
    /// Republic of Cameroon
    #[serde(rename = "CMR")]
    Cmr,
    /// Democratic Republic of the Congo
    #[serde(rename = "COD")]
    Cod,
    /// Republic of the Congo
    #[serde(rename = "COG")]
    Cog,
    /// Cook Islands
    #[serde(rename = "COK")]
    Cok,
    /// Republic of Colombia
    #[serde(rename = "COL")]
    Col,
    /// Union of the Comoros
    #[serde(rename = "COM")]
    Com,
    /// Clipperton Island
    #[serde(rename = "CPT")]
    Cpt,
    /// Republic of Cabo Verde
    #[serde(rename = "CPV")]
    Cpv,
    /// Republic of Costa Rica
    #[serde(rename = "CRI")]
    Cri,
    /// Republic of Cuba
    #[serde(rename = "CUB")]
    Cub,
    /// Curaçao
    #[serde(rename = "CUW")]
    Cuw,
    /// Territory of Christmas Island
    #[serde(rename = "CXR")]
    Cxr,
    /// Cayman Islands
    #[serde(rename = "CYM")]
    Cym,
    /// Republic of Cyprus
    #[serde(rename = "CYP")]
    Cyp,
    /// Czech Republic
    #[serde(rename = "CZE")]
    Cze,
    /// Federal Republic of Germany
    #[serde(rename = "DEU")]
    Deu,
    /// Diego Garcia
    #[serde(rename = "DGA")]
    Dga,
    /// Republic of Djibouti
    #[serde(rename = "DJI")]
    Dji,
    /// Commonwealth of Dominica
    #[serde(rename = "DMA")]
    Dma,
    /// Kingdom of Denmark
    #[serde(rename = "DNK")]
    Dnk,
    /// Dominican Republic
    #[serde(rename = "DOM")]
    Dom,
    /// People's Democratic Republic of Algeria
    #[serde(rename = "DZA")]
    Dza,
    /// Republic of Ecuador
    #[serde(rename = "ECU")]
    Ecu,
    /// Arab Republic of Egypt
    #[serde(rename = "EGY")]
    Egy,
    /// State of Eritrea
    #[serde(rename = "ERI")]
    Eri,
    /// Western Sahara
    #[serde(rename = "ESH")]
    Esh,
    /// Kingdom of Spain
    #[serde(rename = "ESP")]
    Esp,
    /// Republic of Estonia
    #[serde(rename = "EST")]
    Est,
    /// Federal Democratic Republic of Ethiopia
    #[serde(rename = "ETH")]
    Eth,
    /// Republic of Finland
    #[serde(rename = "FIN")]
    Fin,
    /// Republic of Fiji
    #[serde(rename = "FJI")]
    Fji,
    /// Falkland Islands (Islas Malvinas)
    #[serde(rename = "FLK")]
    Flk,
    /// French Republic
    #[serde(rename = "FRA")]
    Fra,
    /// Faroe Islands
    #[serde(rename = "FRO")]
    Fro,
    /// Federated States of Micronesia
    #[serde(rename = "FSM")]
    Fsm,
    /// Gabonese Republic
    #[serde(rename = "GAB")]
    Gab,
    /// United Kingdom of Great Britain and Northern Ireland
    #[serde(rename = "GBR")]
    Gbr,
    /// Georgia
    #[serde(rename = "GEO")]
    Geo,
    /// Bailiwick of Guernsey
    #[serde(rename = "GGY")]
    Ggy,
    /// Republic of Ghana
    #[serde(rename = "GHA")]
    Gha,
    /// Gibraltar
    #[serde(rename = "GIB")]
    Gib,
    /// Republic of Guinea
    #[serde(rename = "GIN")]
    Gin,
    /// Region of Guadeloupe
    #[serde(rename = "GLP")]
    Glp,
    /// Republic of The Gambia
    #[serde(rename = "GMB")]
    Gmb,
    /// Republic of Guinea-Bissau
    #[serde(rename = "GNB")]
    Gnb,
    /// Republic of Equatorial Guinea
    #[serde(rename = "GNQ")]
    Gnq,
    /// Hellenic Republic
    #[serde(rename = "GRC")]
    Grc,
    /// Grenada
    #[serde(rename = "GRD")]
    Grd,
    /// Greenland
    #[serde(rename = "GRL")]
    Grl,
    /// Republic of Guatemala
    #[serde(rename = "GTM")]
    Gtm,
    /// Territorial Collectivity of Guiana
    #[serde(rename = "GUF")]
    Guf,
    /// Territory of Guam
    #[serde(rename = "GUM")]
    Gum,
    /// Co-operative Republic of Guyana
    #[serde(rename = "GUY")]
    Guy,
    /// Hong Kong Special Administrative Region
    #[serde(rename = "HKG")]
    Hkg,
    /// Territory of Heard Island and McDonald Islands
    #[serde(rename = "HMD")]
    Hmd,
    /// Republic of Honduras
    #[serde(rename = "HND")]
    Hnd,
    /// Republic of Croatia
    #[serde(rename = "HRV")]
    Hrv,
    /// Republic of Haiti
    #[serde(rename = "HTI")]
    Hti,
    /// Hungary
    #[serde(rename = "HUN")]
    Hun,
    /// Republic of Indonesia
    #[serde(rename = "IDN")]
    Idn,
    /// Isle of Man
    #[serde(rename = "IMN")]
    Imn,
    /// Republic of India
    #[serde(rename = "IND")]
    Ind,
    /// British Indian Ocean Territory
    #[serde(rename = "IOT")]
    Iot,
    /// Ireland
    #[serde(rename = "IRL")]
    Irl,
    /// Islamic Republic of Iran
    #[serde(rename = "IRN")]
    Irn,
    /// Republic of Iraq
    #[serde(rename = "IRQ")]
    Irq,
    /// Republic of Iceland
    #[serde(rename = "ISL")]
    Isl,
    /// State of Israel
    #[serde(rename = "ISR")]
    Isr,
    /// Italian Republic
    #[serde(rename = "ITA")]
    Ita,
    /// Jamaica
    #[serde(rename = "JAM")]
    Jam,
    /// Bailiwick of Jersey
    #[serde(rename = "JEY")]
    Jey,
    /// Hashemite Kingdom of Jordan
    #[serde(rename = "JOR")]
    Jor,
    /// Japan
    #[serde(rename = "JPN")]
    Jpn,
    /// Republic of Kazakhstan
    #[serde(rename = "KAZ")]
    Kaz,
    /// Republic of Kenya
    #[serde(rename = "KEN")]
    Ken,
    /// Kyrgyz Republic
    #[serde(rename = "KGZ")]
    Kgz,
    /// Kingdom of Cambodia
    #[serde(rename = "KHM")]
    Khm,
    /// Republic of Kiribati
    #[serde(rename = "KIR")]
    Kir,
    /// Federation of Saint Kitts and Nevis
    #[serde(rename = "KNA")]
    Kna,
    /// Republic of Korea
    #[serde(rename = "KOR")]
    Kor,
    /// State of Kuwait
    #[serde(rename = "KWT")]
    Kwt,
    /// Lao People's Democratic Republic
    #[serde(rename = "LAO")]
    Lao,
    /// Lebanese Republic
    #[serde(rename = "LBN")]
    Lbn,
    /// Republic of Liberia
    #[serde(rename = "LBR")]
    Lbr,
    /// State of Libya
    #[serde(rename = "LBY")]
    Lby,
    /// Saint Lucia
    #[serde(rename = "LCA")]
    Lca,
    /// Principality of Liechtenstein
    #[serde(rename = "LIE")]
    Lie,
    /// Democratic Socialist Republic of Sri Lanka
    #[serde(rename = "LKA")]
    Lka,
    /// Kingdom of Lesotho
    #[serde(rename = "LSO")]
    Lso,
    /// Republic of Lithuania
    #[serde(rename = "LTU")]
    Ltu,
    /// Grand Duchy of Luxembourg
    #[serde(rename = "LUX")]
    Lux,
    /// Republic of Latvia
    #[serde(rename = "LVA")]
    Lva,
    /// Macau Special Administrative Region
    #[serde(rename = "MAC")]
    Mac,
    /// Saint Martin
    #[serde(rename = "MAF")]
    Maf,
    /// Kingdom of Morocco
    #[serde(rename = "MAR")]
    Mar,
    /// Principality of Monaco
    #[serde(rename = "MCO")]
    Mco,
    /// Republic of Moldova
    #[serde(rename = "MDA")]
    Mda,
    /// Republic of Madagascar
    #[serde(rename = "MDG")]
    Mdg,
    /// Republic of Maldives
    #[serde(rename = "MDV")]
    Mdv,
    /// United Mexican States
    #[serde(rename = "MEX")]
    Mex,
    /// Republic of the Marshall Islands
    #[serde(rename = "MHL")]
    Mhl,
    /// Republic of North Macedonia
    #[serde(rename = "MKD")]
    Mkd,
    /// Republic of Mali
    #[serde(rename = "MLI")]
    Mli,
    /// Republic of Malta
    #[serde(rename = "MLT")]
    Mlt,
    /// Union of Burma
    #[serde(rename = "MMR")]
    Mmr,
    /// Montenegro
    #[serde(rename = "MNE")]
    Mne,
    /// Mongolia
    #[serde(rename = "MNG")]
    Mng,
    /// Commonwealth of the Northern Mariana Islands
    #[serde(rename = "MNP")]
    Mnp,
    /// Republic of Mozambique
    #[serde(rename = "MOZ")]
    Moz,
    /// Islamic Republic of Mauritania
    #[serde(rename = "MRT")]
    Mrt,
    /// Montserrat
    #[serde(rename = "MSR")]
    Msr,
    /// Territorial Collectivity of Martinique
    #[serde(rename = "MTQ")]
    Mtq,
    /// Republic of Mauritius
    #[serde(rename = "MUS")]
    Mus,
    /// Republic of Malawi
    #[serde(rename = "MWI")]
    Mwi,
    /// Malaysia
    #[serde(rename = "MYS")]
    Mys,
    /// Department of Mayotte
    #[serde(rename = "MYT")]
    Myt,
    /// Republic of Namibia
    #[serde(rename = "NAM")]
    Nam,
    /// New Caledonia
    #[serde(rename = "NCL")]
    Ncl,
    /// Republic of Niger
    #[serde(rename = "NER")]
    Ner,
    /// Territory of Norfolk Island
    #[serde(rename = "NFK")]
    Nfk,
    /// Federal Republic of Nigeria
    #[serde(rename = "NGA")]
    Nga,
    /// Republic of Nicaragua
    #[serde(rename = "NIC")]
    Nic,
    /// Niue
    #[serde(rename = "NIU")]
    Niu,
    /// Kingdom of the Netherlands
    #[serde(rename = "NLD")]
    Nld,
    /// Kingdom of Norway
    #[serde(rename = "NOR")]
    Nor,
    /// Federal Democratic Republic of Nepal
    #[serde(rename = "NPL")]
    Npl,
    /// Republic of Nauru
    #[serde(rename = "NRU")]
    Nru,
    /// New Zealand
    #[serde(rename = "NZL")]
    Nzl,
    /// Sultanate of Oman
    #[serde(rename = "OMN")]
    Omn,
    /// Islamic Republic of Pakistan
    #[serde(rename = "PAK")]
    Pak,
    /// Republic of Panama
    #[serde(rename = "PAN")]
    Pan,
    /// Pitcairn, Henderson, Ducie, and Oeno Islands
    #[serde(rename = "PCN")]
    Pcn,
    /// Republic of Peru
    #[serde(rename = "PER")]
    Per,
    /// Republic of the Philippines
    #[serde(rename = "PHL")]
    Phl,
    /// Republic of Palau
    #[serde(rename = "PLW")]
    Plw,
    /// Independent State of Papua New Guinea
    #[serde(rename = "PNG")]
    Png,
    /// Republic of Poland
    #[serde(rename = "POL")]
    Pol,
    /// Commonwealth of Puerto Rico
    #[serde(rename = "PRI")]
    Pri,
    /// Democratic People's Republic of Korea
    #[serde(rename = "PRK")]
    Prk,
    /// Portuguese Republic
    #[serde(rename = "PRT")]
    Prt,
    /// Republic of Paraguay
    #[serde(rename = "PRY")]
    Pry,
    /// French Polynesia
    #[serde(rename = "PYF")]
    Pyf,
    /// State of Qatar
    #[serde(rename = "QAT")]
    Qat,
    /// Region of Reunion
    #[serde(rename = "REU")]
    Reu,
    /// Romania
    #[serde(rename = "ROU")]
    Rou,
    /// Russian Federation
    #[serde(rename = "RUS")]
    Rus,
    /// Republic of Rwanda
    #[serde(rename = "RWA")]
    Rwa,
    /// Kingdom of Saudi Arabia
    #[serde(rename = "SAU")]
    Sau,
    /// Republic of the Sudan
    #[serde(rename = "SDN")]
    Sdn,
    /// Republic of Senegal
    #[serde(rename = "SEN")]
    Sen,
    /// Republic of Singapore
    #[serde(rename = "SGP")]
    Sgp,
    /// South Georgia and the South Sandwich Islands
    #[serde(rename = "SGS")]
    Sgs,
    /// Saint Helena, Ascension, and Tristan da Cunha
    #[serde(rename = "SHN")]
    Shn,
    /// Solomon Islands
    #[serde(rename = "SLB")]
    Slb,
    /// Republic of Sierra Leone
    #[serde(rename = "SLE")]
    Sle,
    /// Republic of El Salvador
    #[serde(rename = "SLV")]
    Slv,
    /// Republic of San Marino
    #[serde(rename = "SMR")]
    Smr,
    /// Federal Republic of Somalia
    #[serde(rename = "SOM")]
    Som,
    /// Territorial Collectivity of Saint Pierre and Miquelon
    #[serde(rename = "SPM")]
    Spm,
    /// Republic of Serbia
    #[serde(rename = "SRB")]
    Srb,
    /// Republic of South Sudan
    #[serde(rename = "SSD")]
    Ssd,
    /// Democratic Republic of Sao Tome and Principe
    #[serde(rename = "STP")]
    Stp,
    /// Republic of Suriname
    #[serde(rename = "SUR")]
    Sur,
    /// Slovak Republic
    #[serde(rename = "SVK")]
    Svk,
    /// Republic of Slovenia
    #[serde(rename = "SVN")]
    Svn,
    /// Kingdom of Sweden
    #[serde(rename = "SWE")]
    Swe,
    /// Kingdom of Eswatini
    #[serde(rename = "SWZ")]
    Swz,
    /// Sint Maarten
    #[serde(rename = "SXM")]
    Sxm,
    /// Republic of Seychelles
    #[serde(rename = "SYC")]
    Syc,
    /// Syrian Arab Republic
    #[serde(rename = "SYR")]
    Syr,
    /// Turks and Caicos Islands
    #[serde(rename = "TCA")]
    Tca,
    /// Republic of Chad
    #[serde(rename = "TCD")]
    Tcd,
    /// Togolese Republic
    #[serde(rename = "TGO")]
    Tgo,
    /// Kingdom of Thailand
    #[serde(rename = "THA")]
    Tha,
    /// Republic of Tajikistan
    #[serde(rename = "TJK")]
    Tjk,
    /// Tokelau
    #[serde(rename = "TKL")]
    Tkl,
    /// Turkmenistan
    #[serde(rename = "TKM")]
    Tkm,
    /// Democratic Republic of Timor-Leste
    #[serde(rename = "TLS")]
    Tls,
    /// Kingdom of Tonga
    #[serde(rename = "TON")]
    Ton,
    /// Republic of Trinidad and Tobago
    #[serde(rename = "TTO")]
    Tto,
    /// Republic of Tunisia
    #[serde(rename = "TUN")]
    Tun,
    /// Republic of Turkey
    #[serde(rename = "TUR")]
    Tur,
    /// Tuvalu
    #[serde(rename = "TUV")]
    Tuv,
    /// Taiwan
    #[serde(rename = "TWN")]
    Twn,
    /// United Republic of Tanzania
    #[serde(rename = "TZA")]
    Tza,
    /// Republic of Uganda
    #[serde(rename = "UGA")]
    Uga,
    /// Ukraine
    #[serde(rename = "UKR")]
    Ukr,
    /// Oriental Republic of Uruguay
    #[serde(rename = "URY")]
    Ury,
    /// Republic of Uzbekistan
    #[serde(rename = "UZB")]
    Uzb,
    /// State of the Vatican City
    #[serde(rename = "VAT")]
    Vat,
    /// Saint Vincent and the Grenadines
    #[serde(rename = "VCT")]
    Vct,
    /// Bolivarian Republic of Venezuela
    #[serde(rename = "VEN")]
    Ven,
    /// British Virgin Islands
    #[serde(rename = "VGB")]
    Vgb,
    /// United States Virgin Islands
    #[serde(rename = "VIR")]
    Vir,
    /// Socialist Republic of Vietnam
    #[serde(rename = "VNM")]
    Vnm,
    /// Republic of Vanuatu
    #[serde(rename = "VUT")]
    Vut,
    /// Wallis and Futuna
    #[serde(rename = "WLF")]
    Wlf,
    /// Independent State of Samoa
    #[serde(rename = "WSM")]
    Wsm,
    /// Territory of Ashmore and Cartier Islands
    #[serde(rename = "XAC")]
    Xac,
    /// Entity 1
    #[serde(rename = "XAZ")]
    Xaz,
    /// Bassas da India
    #[serde(rename = "XBI")]
    Xbi,
    /// Baker Island
    #[serde(rename = "XBK")]
    Xbk,
    /// Entity 2
    #[serde(rename = "XCR")]
    Xcr,
    /// Coral Sea Islands Territory
    #[serde(rename = "XCS")]
    Xcs,
    /// Entity 3
    #[serde(rename = "XCY")]
    Xcy,
    /// Europa Island
    #[serde(rename = "XEU")]
    Xeu,
    /// Glorioso Islands
    #[serde(rename = "XGL")]
    Xgl,
    /// Gaza Strip
    #[serde(rename = "XGZ")]
    Xgz,
    /// Howland Island
    #[serde(rename = "XHO")]
    Xho,
    /// Johnston Atoll
    #[serde(rename = "XJA")]
    Xja,
    /// Jan Mayen
    #[serde(rename = "XJM")]
    Xjm,
    /// Juan de Nova Island
    #[serde(rename = "XJN")]
    Xjn,
    /// Jarvis Island
    #[serde(rename = "XJV")]
    Xjv,
    /// Entity 4
    #[serde(rename = "XKM")]
    Xkm,
    /// Entity 5
    #[serde(rename = "XKN")]
    Xkn,
    /// Kingman Reef
    #[serde(rename = "XKR")]
    Xkr,
    /// Republic of Kosovo
    #[serde(rename = "XKS")]
    Xks,
    /// Midway Islands
    #[serde(rename = "XMW")]
    Xmw,
    /// Navassa Island
    #[serde(rename = "XNV")]
    Xnv,
    /// Palmyra Atoll
    #[serde(rename = "XPL")]
    Xpl,
    /// Paracel Islands
    #[serde(rename = "XPR")]
    Xpr,
    /// Akrotiri
    #[serde(rename = "XQZ")]
    Xqz,
    /// Spratly Islands
    #[serde(rename = "XSP")]
    Xsp,
    /// Svalbard
    #[serde(rename = "XSV")]
    Xsv,
    /// Tromelin Island
    #[serde(rename = "XTR")]
    Xtr,
    /// West Bank
    #[serde(rename = "XWB")]
    Xwb,
    /// Wake Island
    #[serde(rename = "XWK")]
    Xwk,
    /// Dhekelia
    #[serde(rename = "XXD")]
    Xxd,
    /// Republic of Yemen
    #[serde(rename = "YEM")]
    Yem,
    /// Republic of South Africa
    #[serde(rename = "ZAF")]
    Zaf,
    /// Republic of Zambia
    #[serde(rename = "ZMB")]
    Zmb,
    /// Republic of Zimbabwe
    #[serde(rename = "ZWE")]
    Zwe,
    /// FOUR EYES
    #[serde(rename = "ACGU")]
    Acgu,
    /// AFRICOM Multinational Strategic Partners
    #[serde(rename = "AMSP")]
    Amsp,
    /// Athens Olympics Security Coalition
    #[serde(rename = "AOSC")]
    Aosc,
    /// African Peacekeeping Force Somalia
    #[serde(rename = "APFS")]
    Apfs,
    /// Association of Southeast Asian Nations (ASEAN)
    #[serde(rename = "ASEA")]
    Asea,
    /// Australia Group
    #[serde(rename = "AUSTRALIA_GROUP")]
    AustraliaGroup,
    /// Boko Haram Task Force
    #[serde(rename = "BHTF")]
    Bhtf,
    /// Biological Weapons Convention States
    #[serde(rename = "BWCS")]
    Bwcs,
    /// Combined Forces Command Korea
    #[serde(rename = "CFCK")]
    Cfck,
    /// Coalition Forces Odyssey Dawn
    #[serde(rename = "CFOD")]
    Cfod,
    /// Coalition Forces Unified Protector
    #[serde(rename = "CFUP")]
    Cfup,
    /// Combined Libya Fusion Cell
    #[serde(rename = "CLFC")]
    Clfc,
    /// Combined Maritime Forces Central
    #[serde(rename = "CMFC")]
    Cmfc,
    /// Cooperative Maritime Forces Pacific
    #[serde(rename = "CMFP")]
    Cmfp,
    /// Civilian Protection Monitoring Team for Sudan
    #[serde(rename = "CPMT")]
    Cpmt,
    /// Countering Transnational Organized Crime
    #[serde(rename = "CTOC")]
    Ctoc,
    /// Chemical Weapons Convention States
    #[serde(rename = "CWCS")]
    Cwcs,
    /// European Counter-Terrorism Forces
    #[serde(rename = "ECTF")]
    Ectf,
    /// European Union Stabilization Forces in Bosnia
    #[serde(rename = "EFOR")]
    Efor,
    /// European Union
    #[serde(rename = "EU")]
    Eu,
    /// European Union DARFUR
    #[serde(rename = "EUDA")]
    Euda,
    /// Counter Violent Extremist Organizations Framework Partners
    #[serde(rename = "FRME")]
    Frme,
    /// FIVE EYES
    #[serde(rename = "FVEY")]
    Fvey,
    /// Gulf Cooperation Council
    #[serde(rename = "GCCH")]
    Gcch,
    /// Global Counter-Terrorism Forces
    #[serde(rename = "GCTF")]
    Gctf,
    /// Global Foreign Terrorist Fighter Network Exchange
    #[serde(rename = "GFNX")]
    Gfnx,
    /// Global Maritime Interception Forces
    #[serde(rename = "GMIF")]
    Gmif,
    /// International Events Security Coalition
    #[serde(rename = "IESC")]
    Iesc,
    /// International Maritime Security Construct
    #[serde(rename = "IMSC")]
    Imsc,
    /// INDOPACOM Multinational Strategic Partners
    #[serde(rename = "IMSP")]
    Imsp,
    /// INDO PACIFIC Maritime Call
    #[serde(rename = "IPMC")]
    Ipmc,
    /// Inherent Resolve Kinetic Support
    #[serde(rename = "IRKS")]
    Irks,
    /// International Security Assistance Force for Afghanistan
    #[serde(rename = "ISAF")]
    Isaf,
    /// International Syria Support Group
    #[serde(rename = "ISSG")]
    Issg,
    /// Stabilization Forces in Kosovo
    #[serde(rename = "KFOR")]
    Kfor,
    /// Multinational Coalition Forces-Iraq
    #[serde(rename = "MCFI")]
    Mcfi,
    /// Middle East Stability Force
    #[serde(rename = "MESF")]
    Mesf,
    /// Multinational GEOINT Europe
    #[serde(rename = "MGEU")]
    Mgeu,
    /// Multinational Interim Force Haiti
    #[serde(rename = "MIFH")]
    Mifh,
    /// Multi-Lateral Enduring Contingency
    #[serde(rename = "MLEC")]
    Mlec,
    /// Multinational Task Force
    #[serde(rename = "MNTF")]
    Mntf,
    /// Multinational Peacekeeping Forces
    #[serde(rename = "MPFL")]
    Mpfl,
    /// North African Counter-Terrorism Forces
    #[serde(rename = "NACT")]
    Nact,
    /// North Atlantic Treaty Organization
    #[serde(rename = "NATO")]
    Nato,
    /// NATO CFE Treaty on Conventional Armed Forces in Europe
    #[serde(rename = "NCFE")]
    Ncfe,
    /// North Korea Intelligence Coalition
    #[serde(rename = "NKIC")]
    Nkic,
    /// NORDIC
    #[serde(rename = "NRDC")]
    Nrdc,
    /// Nuclear Suppliers' Group
    #[serde(rename = "NSG")]
    Nsg,
    /// Olympic Security Advisory Group
    #[serde(rename = "OSAG")]
    Osag,
    /// Open Skies Treaty
    #[serde(rename = "OSTY")]
    Osty,
    /// Partnership for Actions in West Africa
    #[serde(rename = "PAWA")]
    Pawa,
    /// Persian Gulf Multinational Forces
    #[serde(rename = "PGMF")]
    Pgmf,
    /// Pacific Security Monitoring Exchange
    #[serde(rename = "PSMX")]
    Psmx,
    /// Russia Intelligence Sharing Coalition
    #[serde(rename = "RISC")]
    Risc,
    /// Resolute Support Mission Afghanistan
    #[serde(rename = "RSMA")]
    Rsma,
    /// Stabilization Force
    #[serde(rename = "SFOR")]
    Sfor,
    /// Special Operations Forces Partners
    #[serde(rename = "SOFP")]
    Sofp,
    /// SOF Planning Activities in Afghanistan (also called "11-Eyes")
    #[serde(rename = "SPAA")]
    Spaa,
    /// THREE EYES
    #[serde(rename = "TEYE")]
    Teye,
    /// Terrorist Financing Targeting Center
    #[serde(rename = "TFTC")]
    Tftc,
    /// United Nations Command, Korea
    #[serde(rename = "UNCK")]
    Unck,
}

/// CVEnumISMCATFGIProtected Values
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FgiSourceProtectedEnum {
    /// Foreign Government Information
    #[serde(rename = "FGI")]
    Fgi,
    /// Aruba
    #[serde(rename = "ABW")]
    Abw,
    /// Islamic Republic of Afghanistan
    #[serde(rename = "AFG")]
    Afg,
    /// Republic of Angola
    #[serde(rename = "AGO")]
    Ago,
    /// Anguilla
    #[serde(rename = "AIA")]
    Aia,
    /// Republic of Albania
    #[serde(rename = "ALB")]
    Alb,
    /// Principality of Andorra
    #[serde(rename = "AND")]
    And,
    /// United Arab Emirates
    #[serde(rename = "ARE")]
    Are,
    /// Argentine Republic
    #[serde(rename = "ARG")]
    Arg,
    /// Republic of Armenia
    #[serde(rename = "ARM")]
    Arm,
    /// Territory of American Samoa
    #[serde(rename = "ASM")]
    Asm,
    /// Antarctica
    #[serde(rename = "ATA")]
    Ata,
    /// French Southern and Antarctic Lands
    #[serde(rename = "ATF")]
    Atf,
    /// Antigua and Barbuda
    #[serde(rename = "ATG")]
    Atg,
    /// Commonwealth of Australia
    #[serde(rename = "AUS")]
    Aus,
    /// Republic of Austria
    #[serde(rename = "AUT")]
    Aut,
    /// Guantanamo Bay Naval Base
    #[serde(rename = "AX2")]
    Ax2,
    /// Entity 6
    #[serde(rename = "AX3")]
    Ax3,
    /// Republic of Azerbaijan
    #[serde(rename = "AZE")]
    Aze,
    /// Republic of Burundi
    #[serde(rename = "BDI")]
    Bdi,
    /// Kingdom of Belgium
    #[serde(rename = "BEL")]
    Bel,
    /// Republic of Benin
    #[serde(rename = "BEN")]
    Ben,
    /// Bonaire, Sint Eustatius, and Saba
    #[serde(rename = "BES")]
    Bes,
    /// Burkina Faso
    #[serde(rename = "BFA")]
    Bfa,
    /// People's Republic of Bangladesh
    #[serde(rename = "BGD")]
    Bgd,
    /// Republic of Bulgaria
    #[serde(rename = "BGR")]
    Bgr,
    /// Kingdom of Bahrain
    #[serde(rename = "BHR")]
    Bhr,
    /// Commonwealth of The Bahamas
    #[serde(rename = "BHS")]
    Bhs,
    /// Bosnia and Herzegovina
    #[serde(rename = "BIH")]
    Bih,
    /// Saint Barthelemy
    #[serde(rename = "BLM")]
    Blm,
    /// Republic of Belarus
    #[serde(rename = "BLR")]
    Blr,
    /// Belize
    #[serde(rename = "BLZ")]
    Blz,
    /// Bermuda
    #[serde(rename = "BMU")]
    Bmu,
    /// Plurinational State of Bolivia
    #[serde(rename = "BOL")]
    Bol,
    /// Federative Republic of Brazil
    #[serde(rename = "BRA")]
    Bra,
    /// Barbados
    #[serde(rename = "BRB")]
    Brb,
    /// Brunei Darussalam
    #[serde(rename = "BRN")]
    Brn,
    /// Kingdom of Bhutan
    #[serde(rename = "BTN")]
    Btn,
    /// Bouvet Island
    #[serde(rename = "BVT")]
    Bvt,
    /// Republic of Botswana
    #[serde(rename = "BWA")]
    Bwa,
    /// Central African Republic
    #[serde(rename = "CAF")]
    Caf,
    /// Canada
    #[serde(rename = "CAN")]
    Can,
    /// Territory of Cocos (Keeling) Islands
    #[serde(rename = "CCK")]
    Cck,
    /// Swiss Confederation
    #[serde(rename = "CHE")]
    Che,
    /// Republic of Chile
    #[serde(rename = "CHL")]
    Chl,
    /// People's Republic of China
    #[serde(rename = "CHN")]
    Chn,
    /// Republic of Côte d'Ivoire
    #[serde(rename = "CIV")]
    Civ,
    /// Republic of Cameroon
    #[serde(rename = "CMR")]
    Cmr,
    /// Democratic Republic of the Congo
    #[serde(rename = "COD")]
    Cod,
    /// Republic of the Congo
    #[serde(rename = "COG")]
    Cog,
    /// Cook Islands
    #[serde(rename = "COK")]
    Cok,
    /// Republic of Colombia
    #[serde(rename = "COL")]
    Col,
    /// Union of the Comoros
    #[serde(rename = "COM")]
    Com,
    /// Clipperton Island
    #[serde(rename = "CPT")]
    Cpt,
    /// Republic of Cabo Verde
    #[serde(rename = "CPV")]
    Cpv,
    /// Republic of Costa Rica
    #[serde(rename = "CRI")]
    Cri,
    /// Republic of Cuba
    #[serde(rename = "CUB")]
    Cub,
    /// Curaçao
    #[serde(rename = "CUW")]
    Cuw,
    /// Territory of Christmas Island
    #[serde(rename = "CXR")]
    Cxr,
    /// Cayman Islands
    #[serde(rename = "CYM")]
    Cym,
    /// Republic of Cyprus
    #[serde(rename = "CYP")]
    Cyp,
    /// Czech Republic
    #[serde(rename = "CZE")]
    Cze,
    /// Federal Republic of Germany
    #[serde(rename = "DEU")]
    Deu,
    /// Diego Garcia
    #[serde(rename = "DGA")]
    Dga,
    /// Republic of Djibouti
    #[serde(rename = "DJI")]
    Dji,
    /// Commonwealth of Dominica
    #[serde(rename = "DMA")]
    Dma,
    /// Kingdom of Denmark
    #[serde(rename = "DNK")]
    Dnk,
    /// Dominican Republic
    #[serde(rename = "DOM")]
    Dom,
    /// People's Democratic Republic of Algeria
    #[serde(rename = "DZA")]
    Dza,
    /// Republic of Ecuador
    #[serde(rename = "ECU")]
    Ecu,
    /// Arab Republic of Egypt
    #[serde(rename = "EGY")]
    Egy,
    /// State of Eritrea
    #[serde(rename = "ERI")]
    Eri,
    /// Western Sahara
    #[serde(rename = "ESH")]
    Esh,
    /// Kingdom of Spain
    #[serde(rename = "ESP")]
    Esp,
    /// Republic of Estonia
    #[serde(rename = "EST")]
    Est,
    /// Federal Democratic Republic of Ethiopia
    #[serde(rename = "ETH")]
    Eth,
    /// Republic of Finland
    #[serde(rename = "FIN")]
    Fin,
    /// Republic of Fiji
    #[serde(rename = "FJI")]
    Fji,
    /// Falkland Islands (Islas Malvinas)
    #[serde(rename = "FLK")]
    Flk,
    /// French Republic
    #[serde(rename = "FRA")]
    Fra,
    /// Faroe Islands
    #[serde(rename = "FRO")]
    Fro,
    /// Federated States of Micronesia
    #[serde(rename = "FSM")]
    Fsm,
    /// Gabonese Republic
    #[serde(rename = "GAB")]
    Gab,
    /// United Kingdom of Great Britain and Northern Ireland
    #[serde(rename = "GBR")]
    Gbr,
    /// Georgia
    #[serde(rename = "GEO")]
    Geo,
    /// Bailiwick of Guernsey
    #[serde(rename = "GGY")]
    Ggy,
    /// Republic of Ghana
    #[serde(rename = "GHA")]
    Gha,
    /// Gibraltar
    #[serde(rename = "GIB")]
    Gib,
    /// Republic of Guinea
    #[serde(rename = "GIN")]
    Gin,
    /// Region of Guadeloupe
    #[serde(rename = "GLP")]
    Glp,
    /// Republic of The Gambia
    #[serde(rename = "GMB")]
    Gmb,
    /// Republic of Guinea-Bissau
    #[serde(rename = "GNB")]
    Gnb,
    /// Republic of Equatorial Guinea
    #[serde(rename = "GNQ")]
    Gnq,
    /// Hellenic Republic
    #[serde(rename = "GRC")]
    Grc,
    /// Grenada
    #[serde(rename = "GRD")]
    Grd,
    /// Greenland
    #[serde(rename = "GRL")]
    Grl,
    /// Republic of Guatemala
    #[serde(rename = "GTM")]
    Gtm,
    /// Territorial Collectivity of Guiana
    #[serde(rename = "GUF")]
    Guf,
    /// Territory of Guam
    #[serde(rename = "GUM")]
    Gum,
    /// Co-operative Republic of Guyana
    #[serde(rename = "GUY")]
    Guy,
    /// Hong Kong Special Administrative Region
    #[serde(rename = "HKG")]
    Hkg,
    /// Territory of Heard Island and McDonald Islands
    #[serde(rename = "HMD")]
    Hmd,
    /// Republic of Honduras
    #[serde(rename = "HND")]
    Hnd,
    /// Republic of Croatia
    #[serde(rename = "HRV")]
    Hrv,
    /// Republic of Haiti
    #[serde(rename = "HTI")]
    Hti,
    /// Hungary
    #[serde(rename = "HUN")]
    Hun,
    /// Republic of Indonesia
    #[serde(rename = "IDN")]
    Idn,
    /// Isle of Man
    #[serde(rename = "IMN")]
    Imn,
    /// Republic of India
    #[serde(rename = "IND")]
    Ind,
    /// British Indian Ocean Territory
    #[serde(rename = "IOT")]
    Iot,
    /// Ireland
    #[serde(rename = "IRL")]
    Irl,
    /// Islamic Republic of Iran
    #[serde(rename = "IRN")]
    Irn,
    /// Republic of Iraq
    #[serde(rename = "IRQ")]
    Irq,
    /// Republic of Iceland
    #[serde(rename = "ISL")]
    Isl,
    /// State of Israel
    #[serde(rename = "ISR")]
    Isr,
    /// Italian Republic
    #[serde(rename = "ITA")]
    Ita,
    /// Jamaica
    #[serde(rename = "JAM")]
    Jam,
    /// Bailiwick of Jersey
    #[serde(rename = "JEY")]
    Jey,
    /// Hashemite Kingdom of Jordan
    #[serde(rename = "JOR")]
    Jor,
    /// Japan
    #[serde(rename = "JPN")]
    Jpn,
    /// Republic of Kazakhstan
    #[serde(rename = "KAZ")]
    Kaz,
    /// Republic of Kenya
    #[serde(rename = "KEN")]
    Ken,
    /// Kyrgyz Republic
    #[serde(rename = "KGZ")]
    Kgz,
    /// Kingdom of Cambodia
    #[serde(rename = "KHM")]
    Khm,
    /// Republic of Kiribati
    #[serde(rename = "KIR")]
    Kir,
    /// Federation of Saint Kitts and Nevis
    #[serde(rename = "KNA")]
    Kna,
    /// Republic of Korea
    #[serde(rename = "KOR")]
    Kor,
    /// State of Kuwait
    #[serde(rename = "KWT")]
    Kwt,
    /// Lao People's Democratic Republic
    #[serde(rename = "LAO")]
    Lao,
    /// Lebanese Republic
    #[serde(rename = "LBN")]
    Lbn,
    /// Republic of Liberia
    #[serde(rename = "LBR")]
    Lbr,
    /// State of Libya
    #[serde(rename = "LBY")]
    Lby,
    /// Saint Lucia
    #[serde(rename = "LCA")]
    Lca,
    /// Principality of Liechtenstein
    #[serde(rename = "LIE")]
    Lie,
    /// Democratic Socialist Republic of Sri Lanka
    #[serde(rename = "LKA")]
    Lka,
    /// Kingdom of Lesotho
    #[serde(rename = "LSO")]
    Lso,
    /// Republic of Lithuania
    #[serde(rename = "LTU")]
    Ltu,
    /// Grand Duchy of Luxembourg
    #[serde(rename = "LUX")]
    Lux,
    /// Republic of Latvia
    #[serde(rename = "LVA")]
    Lva,
    /// Macau Special Administrative Region
    #[serde(rename = "MAC")]
    Mac,
    /// Saint Martin
    #[serde(rename = "MAF")]
    Maf,
    /// Kingdom of Morocco
    #[serde(rename = "MAR")]
    Mar,
    /// Principality of Monaco
    #[serde(rename = "MCO")]
    Mco,
    /// Republic of Moldova
    #[serde(rename = "MDA")]
    Mda,
    /// Republic of Madagascar
    #[serde(rename = "MDG")]
    Mdg,
    /// Republic of Maldives
    #[serde(rename = "MDV")]
    Mdv,
    /// United Mexican States
    #[serde(rename = "MEX")]
    Mex,
    /// Republic of the Marshall Islands
    #[serde(rename = "MHL")]
    Mhl,
    /// Republic of North Macedonia
    #[serde(rename = "MKD")]
    Mkd,
    /// Republic of Mali
    #[serde(rename = "MLI")]
    Mli,
    /// Republic of Malta
    #[serde(rename = "MLT")]
    Mlt,
    /// Union of Burma
    #[serde(rename = "MMR")]
    Mmr,
    /// Montenegro
    #[serde(rename = "MNE")]
    Mne,
    /// Mongolia
    #[serde(rename = "MNG")]
    Mng,
    /// Commonwealth of the Northern Mariana Islands
    #[serde(rename = "MNP")]
    Mnp,
    /// Republic of Mozambique
    #[serde(rename = "MOZ")]
    Moz,
    /// Islamic Republic of Mauritania
    #[serde(rename = "MRT")]
    Mrt,
    /// Montserrat
    #[serde(rename = "MSR")]
    Msr,
    /// Territorial Collectivity of Martinique
    #[serde(rename = "MTQ")]
    Mtq,
    /// Republic of Mauritius
    #[serde(rename = "MUS")]
    Mus,
    /// Republic of Malawi
    #[serde(rename = "MWI")]
    Mwi,
    /// Malaysia
    #[serde(rename = "MYS")]
    Mys,
    /// Department of Mayotte
    #[serde(rename = "MYT")]
    Myt,
    /// Republic of Namibia
    #[serde(rename = "NAM")]
    Nam,
    /// New Caledonia
    #[serde(rename = "NCL")]
    Ncl,
    /// Republic of Niger
    #[serde(rename = "NER")]
    Ner,
    /// Territory of Norfolk Island
    #[serde(rename = "NFK")]
    Nfk,
    /// Federal Republic of Nigeria
    #[serde(rename = "NGA")]
    Nga,
    /// Republic of Nicaragua
    #[serde(rename = "NIC")]
    Nic,
    /// Niue
    #[serde(rename = "NIU")]
    Niu,
    /// Kingdom of the Netherlands
    #[serde(rename = "NLD")]
    Nld,
    /// Kingdom of Norway
    #[serde(rename = "NOR")]
    Nor,
    /// Federal Democratic Republic of Nepal
    #[serde(rename = "NPL")]
    Npl,
    /// Republic of Nauru
    #[serde(rename = "NRU")]
    Nru,
    /// New Zealand
    #[serde(rename = "NZL")]
    Nzl,
    /// Sultanate of Oman
    #[serde(rename = "OMN")]
    Omn,
    /// Islamic Republic of Pakistan
    #[serde(rename = "PAK")]
    Pak,
    /// Republic of Panama
    #[serde(rename = "PAN")]
    Pan,
    /// Pitcairn, Henderson, Ducie, and Oeno Islands
    #[serde(rename = "PCN")]
    Pcn,
    /// Republic of Peru
    #[serde(rename = "PER")]
    Per,
    /// Republic of the Philippines
    #[serde(rename = "PHL")]
    Phl,
    /// Republic of Palau
    #[serde(rename = "PLW")]
    Plw,
    /// Independent State of Papua New Guinea
    #[serde(rename = "PNG")]
    Png,
    /// Republic of Poland
    #[serde(rename = "POL")]
    Pol,
    /// Commonwealth of Puerto Rico
    #[serde(rename = "PRI")]
    Pri,
    /// Democratic People's Republic of Korea
    #[serde(rename = "PRK")]
    Prk,
    /// Portuguese Republic
    #[serde(rename = "PRT")]
    Prt,
    /// Republic of Paraguay
    #[serde(rename = "PRY")]
    Pry,
    /// French Polynesia
    #[serde(rename = "PYF")]
    Pyf,
    /// State of Qatar
    #[serde(rename = "QAT")]
    Qat,
    /// Region of Reunion
    #[serde(rename = "REU")]
    Reu,
    /// Romania
    #[serde(rename = "ROU")]
    Rou,
    /// Russian Federation
    #[serde(rename = "RUS")]
    Rus,
    /// Republic of Rwanda
    #[serde(rename = "RWA")]
    Rwa,
    /// Kingdom of Saudi Arabia
    #[serde(rename = "SAU")]
    Sau,
    /// Republic of the Sudan
    #[serde(rename = "SDN")]
    Sdn,
    /// Republic of Senegal
    #[serde(rename = "SEN")]
    Sen,
    /// Republic of Singapore
    #[serde(rename = "SGP")]
    Sgp,
    /// South Georgia and the South Sandwich Islands
    #[serde(rename = "SGS")]
    Sgs,
    /// Saint Helena, Ascension, and Tristan da Cunha
    #[serde(rename = "SHN")]
    Shn,
    /// Solomon Islands
    #[serde(rename = "SLB")]
    Slb,
    /// Republic of Sierra Leone
    #[serde(rename = "SLE")]
    Sle,
    /// Republic of El Salvador
    #[serde(rename = "SLV")]
    Slv,
    /// Republic of San Marino
    #[serde(rename = "SMR")]
    Smr,
    /// Federal Republic of Somalia
    #[serde(rename = "SOM")]
    Som,
    /// Territorial Collectivity of Saint Pierre and Miquelon
    #[serde(rename = "SPM")]
    Spm,
    /// Republic of Serbia
    #[serde(rename = "SRB")]
    Srb,
    /// Republic of South Sudan
    #[serde(rename = "SSD")]
    Ssd,
    /// Democratic Republic of Sao Tome and Principe
    #[serde(rename = "STP")]
    Stp,
    /// Republic of Suriname
    #[serde(rename = "SUR")]
    Sur,
    /// Slovak Republic
    #[serde(rename = "SVK")]
    Svk,
    /// Republic of Slovenia
    #[serde(rename = "SVN")]
    Svn,
    /// Kingdom of Sweden
    #[serde(rename = "SWE")]
    Swe,
    /// Kingdom of Eswatini
    #[serde(rename = "SWZ")]
    Swz,
    /// Sint Maarten
    #[serde(rename = "SXM")]
    Sxm,
    /// Republic of Seychelles
    #[serde(rename = "SYC")]
    Syc,
    /// Syrian Arab Republic
    #[serde(rename = "SYR")]
    Syr,
    /// Turks and Caicos Islands
    #[serde(rename = "TCA")]
    Tca,
    /// Republic of Chad
    #[serde(rename = "TCD")]
    Tcd,
    /// Togolese Republic
    #[serde(rename = "TGO")]
    Tgo,
    /// Kingdom of Thailand
    #[serde(rename = "THA")]
    Tha,
    /// Republic of Tajikistan
    #[serde(rename = "TJK")]
    Tjk,
    /// Tokelau
    #[serde(rename = "TKL")]
    Tkl,
    /// Turkmenistan
    #[serde(rename = "TKM")]
    Tkm,
    /// Democratic Republic of Timor-Leste
    #[serde(rename = "TLS")]
    Tls,
    /// Kingdom of Tonga
    #[serde(rename = "TON")]
    Ton,
    /// Republic of Trinidad and Tobago
    #[serde(rename = "TTO")]
    Tto,
    /// Republic of Tunisia
    #[serde(rename = "TUN")]
    Tun,
    /// Republic of Turkey
    #[serde(rename = "TUR")]
    Tur,
    /// Tuvalu
    #[serde(rename = "TUV")]
    Tuv,
    /// Taiwan
    #[serde(rename = "TWN")]
    Twn,
    /// United Republic of Tanzania
    #[serde(rename = "TZA")]
    Tza,
    /// Republic of Uganda
    #[serde(rename = "UGA")]
    Uga,
    /// Ukraine
    #[serde(rename = "UKR")]
    Ukr,
    /// Oriental Republic of Uruguay
    #[serde(rename = "URY")]
    Ury,
    /// Republic of Uzbekistan
    #[serde(rename = "UZB")]
    Uzb,
    /// State of the Vatican City
    #[serde(rename = "VAT")]
    Vat,
    /// Saint Vincent and the Grenadines
    #[serde(rename = "VCT")]
    Vct,
    /// Bolivarian Republic of Venezuela
    #[serde(rename = "VEN")]
    Ven,
    /// British Virgin Islands
    #[serde(rename = "VGB")]
    Vgb,
    /// United States Virgin Islands
    #[serde(rename = "VIR")]
    Vir,
    /// Socialist Republic of Vietnam
    #[serde(rename = "VNM")]
    Vnm,
    /// Republic of Vanuatu
    #[serde(rename = "VUT")]
    Vut,
    /// Wallis and Futuna
    #[serde(rename = "WLF")]
    Wlf,
    /// Independent State of Samoa
    #[serde(rename = "WSM")]
    Wsm,
    /// Territory of Ashmore and Cartier Islands
    #[serde(rename = "XAC")]
    Xac,
    /// Entity 1
    #[serde(rename = "XAZ")]
    Xaz,
    /// Bassas da India
    #[serde(rename = "XBI")]
    Xbi,
    /// Baker Island
    #[serde(rename = "XBK")]
    Xbk,
    /// Entity 2
    #[serde(rename = "XCR")]
    Xcr,
    /// Coral Sea Islands Territory
    #[serde(rename = "XCS")]
    Xcs,
    /// Entity 3
    #[serde(rename = "XCY")]
    Xcy,
    /// Europa Island
    #[serde(rename = "XEU")]
    Xeu,
    /// Glorioso Islands
    #[serde(rename = "XGL")]
    Xgl,
    /// Gaza Strip
    #[serde(rename = "XGZ")]
    Xgz,
    /// Howland Island
    #[serde(rename = "XHO")]
    Xho,
    /// Johnston Atoll
    #[serde(rename = "XJA")]
    Xja,
    /// Jan Mayen
    #[serde(rename = "XJM")]
    Xjm,
    /// Juan de Nova Island
    #[serde(rename = "XJN")]
    Xjn,
    /// Jarvis Island
    #[serde(rename = "XJV")]
    Xjv,
    /// Entity 4
    #[serde(rename = "XKM")]
    Xkm,
    /// Entity 5
    #[serde(rename = "XKN")]
    Xkn,
    /// Kingman Reef
    #[serde(rename = "XKR")]
    Xkr,
    /// Republic of Kosovo
    #[serde(rename = "XKS")]
    Xks,
    /// Midway Islands
    #[serde(rename = "XMW")]
    Xmw,
    /// Navassa Island
    #[serde(rename = "XNV")]
    Xnv,
    /// Palmyra Atoll
    #[serde(rename = "XPL")]
    Xpl,
    /// Paracel Islands
    #[serde(rename = "XPR")]
    Xpr,
    /// Akrotiri
    #[serde(rename = "XQZ")]
    Xqz,
    /// Spratly Islands
    #[serde(rename = "XSP")]
    Xsp,
    /// Svalbard
    #[serde(rename = "XSV")]
    Xsv,
    /// Tromelin Island
    #[serde(rename = "XTR")]
    Xtr,
    /// West Bank
    #[serde(rename = "XWB")]
    Xwb,
    /// Wake Island
    #[serde(rename = "XWK")]
    Xwk,
    /// Dhekelia
    #[serde(rename = "XXD")]
    Xxd,
    /// Republic of Yemen
    #[serde(rename = "YEM")]
    Yem,
    /// Republic of South Africa
    #[serde(rename = "ZAF")]
    Zaf,
    /// Republic of Zambia
    #[serde(rename = "ZMB")]
    Zmb,
    /// Republic of Zimbabwe
    #[serde(rename = "ZWE")]
    Zwe,
    /// FOUR EYES
    #[serde(rename = "ACGU")]
    Acgu,
    /// AFRICOM Multinational Strategic Partners
    #[serde(rename = "AMSP")]
    Amsp,
    /// Athens Olympics Security Coalition
    #[serde(rename = "AOSC")]
    Aosc,
    /// African Peacekeeping Force Somalia
    #[serde(rename = "APFS")]
    Apfs,
    /// Association of Southeast Asian Nations (ASEAN)
    #[serde(rename = "ASEA")]
    Asea,
    /// Australia Group
    #[serde(rename = "AUSTRALIA_GROUP")]
    AustraliaGroup,
    /// Boko Haram Task Force
    #[serde(rename = "BHTF")]
    Bhtf,
    /// Biological Weapons Convention States
    #[serde(rename = "BWCS")]
    Bwcs,
    /// Combined Forces Command Korea
    #[serde(rename = "CFCK")]
    Cfck,
    /// Coalition Forces Odyssey Dawn
    #[serde(rename = "CFOD")]
    Cfod,
    /// Coalition Forces Unified Protector
    #[serde(rename = "CFUP")]
    Cfup,
    /// Combined Libya Fusion Cell
    #[serde(rename = "CLFC")]
    Clfc,
    /// Combined Maritime Forces Central
    #[serde(rename = "CMFC")]
    Cmfc,
    /// Cooperative Maritime Forces Pacific
    #[serde(rename = "CMFP")]
    Cmfp,
    /// Civilian Protection Monitoring Team for Sudan
    #[serde(rename = "CPMT")]
    Cpmt,
    /// Countering Transnational Organized Crime
    #[serde(rename = "CTOC")]
    Ctoc,
    /// Chemical Weapons Convention States
    #[serde(rename = "CWCS")]
    Cwcs,
    /// European Counter-Terrorism Forces
    #[serde(rename = "ECTF")]
    Ectf,
    /// European Union Stabilization Forces in Bosnia
    #[serde(rename = "EFOR")]
    Efor,
    /// European Union
    #[serde(rename = "EU")]
    Eu,
    /// European Union DARFUR
    #[serde(rename = "EUDA")]
    Euda,
    /// Counter Violent Extremist Organizations Framework Partners
    #[serde(rename = "FRME")]
    Frme,
    /// FIVE EYES
    #[serde(rename = "FVEY")]
    Fvey,
    /// Gulf Cooperation Council
    #[serde(rename = "GCCH")]
    Gcch,
    /// Global Counter-Terrorism Forces
    #[serde(rename = "GCTF")]
    Gctf,
    /// Global Foreign Terrorist Fighter Network Exchange
    #[serde(rename = "GFNX")]
    Gfnx,
    /// Global Maritime Interception Forces
    #[serde(rename = "GMIF")]
    Gmif,
    /// International Events Security Coalition
    #[serde(rename = "IESC")]
    Iesc,
    /// International Maritime Security Construct
    #[serde(rename = "IMSC")]
    Imsc,
    /// INDOPACOM Multinational Strategic Partners
    #[serde(rename = "IMSP")]
    Imsp,
    /// INDO PACIFIC Maritime Call
    #[serde(rename = "IPMC")]
    Ipmc,
    /// Inherent Resolve Kinetic Support
    #[serde(rename = "IRKS")]
    Irks,
    /// International Security Assistance Force for Afghanistan
    #[serde(rename = "ISAF")]
    Isaf,
    /// International Syria Support Group
    #[serde(rename = "ISSG")]
    Issg,
    /// Stabilization Forces in Kosovo
    #[serde(rename = "KFOR")]
    Kfor,
    /// Multinational Coalition Forces-Iraq
    #[serde(rename = "MCFI")]
    Mcfi,
    /// Middle East Stability Force
    #[serde(rename = "MESF")]
    Mesf,
    /// Multinational GEOINT Europe
    #[serde(rename = "MGEU")]
    Mgeu,
    /// Multinational Interim Force Haiti
    #[serde(rename = "MIFH")]
    Mifh,
    /// Multi-Lateral Enduring Contingency
    #[serde(rename = "MLEC")]
    Mlec,
    /// Multinational Task Force
    #[serde(rename = "MNTF")]
    Mntf,
    /// Multinational Peacekeeping Forces
    #[serde(rename = "MPFL")]
    Mpfl,
    /// North African Counter-Terrorism Forces
    #[serde(rename = "NACT")]
    Nact,
    /// North Atlantic Treaty Organization
    #[serde(rename = "NATO")]
    Nato,
    /// NATO CFE Treaty on Conventional Armed Forces in Europe
    #[serde(rename = "NCFE")]
    Ncfe,
    /// North Korea Intelligence Coalition
    #[serde(rename = "NKIC")]
    Nkic,
    /// NORDIC
    #[serde(rename = "NRDC")]
    Nrdc,
    /// Nuclear Suppliers' Group
    #[serde(rename = "NSG")]
    Nsg,
    /// Olympic Security Advisory Group
    #[serde(rename = "OSAG")]
    Osag,
    /// Open Skies Treaty
    #[serde(rename = "OSTY")]
    Osty,
    /// Partnership for Actions in West Africa
    #[serde(rename = "PAWA")]
    Pawa,
    /// Persian Gulf Multinational Forces
    #[serde(rename = "PGMF")]
    Pgmf,
    /// Pacific Security Monitoring Exchange
    #[serde(rename = "PSMX")]
    Psmx,
    /// Russia Intelligence Sharing Coalition
    #[serde(rename = "RISC")]
    Risc,
    /// Resolute Support Mission Afghanistan
    #[serde(rename = "RSMA")]
    Rsma,
    /// Stabilization Force
    #[serde(rename = "SFOR")]
    Sfor,
    /// Special Operations Forces Partners
    #[serde(rename = "SOFP")]
    Sofp,
    /// SOF Planning Activities in Afghanistan (also called "11-Eyes")
    #[serde(rename = "SPAA")]
    Spaa,
    /// THREE EYES
    #[serde(rename = "TEYE")]
    Teye,
    /// Terrorist Financing Targeting Center
    #[serde(rename = "TFTC")]
    Tftc,
    /// United Nations Command, Korea
    #[serde(rename = "UNCK")]
    Unck,
}

/// (U) The highest classification of any portion that has either OwnerProducer contains NATO or
/// FGI_SourceOpen contains NATO. The HighWaterNATO attribute will be compared against an entity's
/// fineAccessControls NATO value. PERMISSIBLE VALUES The permissible values for this simple type
/// are defined in the Controlled Value Enumeration: CVEnumISMHighWaterNATO.xml
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HighWaterNatoEnum {
    /// NATO UNCLASSIFIED
    #[serde(rename = "NATO_U")]
    NatoU,
    /// NATO RESTRICTED
    #[serde(rename = "NATO_R")]
    NatoR,
    /// NATO CONFIDENTIAL
    #[serde(rename = "NATO_C")]
    NatoC,
    /// NATO SECRET
    #[serde(rename = "NATO_S")]
    NatoS,
    /// NATO TOP SECRET
    #[serde(rename = "NATO_TS")]
    NatoTs,
}

/// Specifies the mode the system is currently operating in.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageModeEnum {
    /// This enumerated value indicates that the message concerns actual systems in real operations;
    /// nothing in the message concerns an exercise and/or a simulated system.
    #[serde(rename = "LIVE")]
    Live,
    /// This enumerated value indicates that the message contained in the envelope concerns actual
    /// systems in an exercise; nothing in the message concerns real operations and/or simulated
    /// systems.
    #[serde(rename = "EXERCISE")]
    Exercise,
    /// This enumerated value indicates that the message contained in the envelope concerns simulated
    /// systems in an exercise; nothing in the message concerns real operations and/or real systems.
    #[serde(rename = "SIMULATION")]
    Simulation,
    /// This enumerated value indicates that the message contained in the envelope concerns simulated
    /// systems when not in an exercise.
    #[serde(rename = "NONEXERCISE_SIMULATION")]
    NonexerciseSimulation,
}

/// North Atlantic Treaty Organization Special Words
pub type NatoSpecialWordsType = String;

/// (U) All currently valid Non-IC markings from the published register. PERMISSIBLE VALUES The
/// permissible values for this simple type are defined in the Controlled Value Enumeration:
/// CVEnumISMNonIC.xml
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NonIcMarkingsEnum {
    /// LIMITED DISTRIBUTION
    #[serde(rename = "DS")]
    Ds,
    /// EXCLUSIVE DISTRIBUTION
    #[serde(rename = "XD")]
    Xd,
    /// NO DISTRIBUTION
    #[serde(rename = "ND")]
    Nd,
    /// SENSITIVE BUT UNCLASSIFIED
    #[serde(rename = "SBU")]
    Sbu,
    /// SENSITIVE BUT UNCLASSIFIED NOFORN
    #[serde(rename = "SBU_NF")]
    SbuNf,
    /// LAW ENFORCEMENT SENSITIVE
    #[serde(rename = "LES")]
    Les,
    /// LAW ENFORCEMENT SENSITIVE NOFORN
    #[serde(rename = "LES_NF")]
    LesNf,
    /// SENSITIVE SECURITY INFORMATION
    #[serde(rename = "SSI")]
    Ssi,
    /// NAVAL NUCLEAR PROPULSION INFORMATION
    #[serde(rename = "NNPI")]
    Nnpi,
}

/// (U) NonUS Control markings supported by ISM PERMISSIBLE VALUES The permissible values for this
/// simple type are defined in the Controlled Value Enumeration: CVEnumISMNonUSControls.xml
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NonUsControlsEnum {
    /// NATO Atomal mark
    #[serde(rename = "NATO_ATOMAL")]
    NatoAtomal,
    /// NATO Bohemia mark
    #[serde(rename = "NATO_BOHEMIA")]
    NatoBohemia,
    /// NATO Balk mark
    #[serde(rename = "NATO_BALK")]
    NatoBalk,
}

/// CVEnumISMCATOwnerProducer Values
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OwnerProducerEnum {
    /// Foreign Government Information
    #[serde(rename = "FGI")]
    Fgi,
    /// Aruba
    #[serde(rename = "ABW")]
    Abw,
    /// Islamic Republic of Afghanistan
    #[serde(rename = "AFG")]
    Afg,
    /// Republic of Angola
    #[serde(rename = "AGO")]
    Ago,
    /// Anguilla
    #[serde(rename = "AIA")]
    Aia,
    /// Republic of Albania
    #[serde(rename = "ALB")]
    Alb,
    /// Principality of Andorra
    #[serde(rename = "AND")]
    And,
    /// United Arab Emirates
    #[serde(rename = "ARE")]
    Are,
    /// Argentine Republic
    #[serde(rename = "ARG")]
    Arg,
    /// Republic of Armenia
    #[serde(rename = "ARM")]
    Arm,
    /// Territory of American Samoa
    #[serde(rename = "ASM")]
    Asm,
    /// Antarctica
    #[serde(rename = "ATA")]
    Ata,
    /// French Southern and Antarctic Lands
    #[serde(rename = "ATF")]
    Atf,
    /// Antigua and Barbuda
    #[serde(rename = "ATG")]
    Atg,
    /// Commonwealth of Australia
    #[serde(rename = "AUS")]
    Aus,
    /// Republic of Austria
    #[serde(rename = "AUT")]
    Aut,
    /// Guantanamo Bay Naval Base
    #[serde(rename = "AX2")]
    Ax2,
    /// Entity 6
    #[serde(rename = "AX3")]
    Ax3,
    /// Republic of Azerbaijan
    #[serde(rename = "AZE")]
    Aze,
    /// Republic of Burundi
    #[serde(rename = "BDI")]
    Bdi,
    /// Kingdom of Belgium
    #[serde(rename = "BEL")]
    Bel,
    /// Republic of Benin
    #[serde(rename = "BEN")]
    Ben,
    /// Bonaire, Sint Eustatius, and Saba
    #[serde(rename = "BES")]
    Bes,
    /// Burkina Faso
    #[serde(rename = "BFA")]
    Bfa,
    /// People's Republic of Bangladesh
    #[serde(rename = "BGD")]
    Bgd,
    /// Republic of Bulgaria
    #[serde(rename = "BGR")]
    Bgr,
    /// Kingdom of Bahrain
    #[serde(rename = "BHR")]
    Bhr,
    /// Commonwealth of The Bahamas
    #[serde(rename = "BHS")]
    Bhs,
    /// Bosnia and Herzegovina
    #[serde(rename = "BIH")]
    Bih,
    /// Saint Barthelemy
    #[serde(rename = "BLM")]
    Blm,
    /// Republic of Belarus
    #[serde(rename = "BLR")]
    Blr,
    /// Belize
    #[serde(rename = "BLZ")]
    Blz,
    /// Bermuda
    #[serde(rename = "BMU")]
    Bmu,
    /// Plurinational State of Bolivia
    #[serde(rename = "BOL")]
    Bol,
    /// Federative Republic of Brazil
    #[serde(rename = "BRA")]
    Bra,
    /// Barbados
    #[serde(rename = "BRB")]
    Brb,
    /// Brunei Darussalam
    #[serde(rename = "BRN")]
    Brn,
    /// Kingdom of Bhutan
    #[serde(rename = "BTN")]
    Btn,
    /// Bouvet Island
    #[serde(rename = "BVT")]
    Bvt,
    /// Republic of Botswana
    #[serde(rename = "BWA")]
    Bwa,
    /// Central African Republic
    #[serde(rename = "CAF")]
    Caf,
    /// Canada
    #[serde(rename = "CAN")]
    Can,
    /// Territory of Cocos (Keeling) Islands
    #[serde(rename = "CCK")]
    Cck,
    /// Swiss Confederation
    #[serde(rename = "CHE")]
    Che,
    /// Republic of Chile
    #[serde(rename = "CHL")]
    Chl,
    /// People's Republic of China
    #[serde(rename = "CHN")]
    Chn,
    /// Republic of Côte d'Ivoire
    #[serde(rename = "CIV")]
    Civ,
    /// Republic of Cameroon
    #[serde(rename = "CMR")]
    Cmr,
    /// Democratic Republic of the Congo
    #[serde(rename = "COD")]
    Cod,
    /// Republic of the Congo
    #[serde(rename = "COG")]
    Cog,
    /// Cook Islands
    #[serde(rename = "COK")]
    Cok,
    /// Republic of Colombia
    #[serde(rename = "COL")]
    Col,
    /// Union of the Comoros
    #[serde(rename = "COM")]
    Com,
    /// Clipperton Island
    #[serde(rename = "CPT")]
    Cpt,
    /// Republic of Cabo Verde
    #[serde(rename = "CPV")]
    Cpv,
    /// Republic of Costa Rica
    #[serde(rename = "CRI")]
    Cri,
    /// Republic of Cuba
    #[serde(rename = "CUB")]
    Cub,
    /// Curaçao
    #[serde(rename = "CUW")]
    Cuw,
    /// Territory of Christmas Island
    #[serde(rename = "CXR")]
    Cxr,
    /// Cayman Islands
    #[serde(rename = "CYM")]
    Cym,
    /// Republic of Cyprus
    #[serde(rename = "CYP")]
    Cyp,
    /// Czech Republic
    #[serde(rename = "CZE")]
    Cze,
    /// Federal Republic of Germany
    #[serde(rename = "DEU")]
    Deu,
    /// Diego Garcia
    #[serde(rename = "DGA")]
    Dga,
    /// Republic of Djibouti
    #[serde(rename = "DJI")]
    Dji,
    /// Commonwealth of Dominica
    #[serde(rename = "DMA")]
    Dma,
    /// Kingdom of Denmark
    #[serde(rename = "DNK")]
    Dnk,
    /// Dominican Republic
    #[serde(rename = "DOM")]
    Dom,
    /// People's Democratic Republic of Algeria
    #[serde(rename = "DZA")]
    Dza,
    /// Republic of Ecuador
    #[serde(rename = "ECU")]
    Ecu,
    /// Arab Republic of Egypt
    #[serde(rename = "EGY")]
    Egy,
    /// State of Eritrea
    #[serde(rename = "ERI")]
    Eri,
    /// Western Sahara
    #[serde(rename = "ESH")]
    Esh,
    /// Kingdom of Spain
    #[serde(rename = "ESP")]
    Esp,
    /// Republic of Estonia
    #[serde(rename = "EST")]
    Est,
    /// Federal Democratic Republic of Ethiopia
    #[serde(rename = "ETH")]
    Eth,
    /// Republic of Finland
    #[serde(rename = "FIN")]
    Fin,
    /// Republic of Fiji
    #[serde(rename = "FJI")]
    Fji,
    /// Falkland Islands (Islas Malvinas)
    #[serde(rename = "FLK")]
    Flk,
    /// French Republic
    #[serde(rename = "FRA")]
    Fra,
    /// Faroe Islands
    #[serde(rename = "FRO")]
    Fro,
    /// Federated States of Micronesia
    #[serde(rename = "FSM")]
    Fsm,
    /// Gabonese Republic
    #[serde(rename = "GAB")]
    Gab,
    /// United Kingdom of Great Britain and Northern Ireland
    #[serde(rename = "GBR")]
    Gbr,
    /// Georgia
    #[serde(rename = "GEO")]
    Geo,
    /// Bailiwick of Guernsey
    #[serde(rename = "GGY")]
    Ggy,
    /// Republic of Ghana
    #[serde(rename = "GHA")]
    Gha,
    /// Gibraltar
    #[serde(rename = "GIB")]
    Gib,
    /// Republic of Guinea
    #[serde(rename = "GIN")]
    Gin,
    /// Region of Guadeloupe
    #[serde(rename = "GLP")]
    Glp,
    /// Republic of The Gambia
    #[serde(rename = "GMB")]
    Gmb,
    /// Republic of Guinea-Bissau
    #[serde(rename = "GNB")]
    Gnb,
    /// Republic of Equatorial Guinea
    #[serde(rename = "GNQ")]
    Gnq,
    /// Hellenic Republic
    #[serde(rename = "GRC")]
    Grc,
    /// Grenada
    #[serde(rename = "GRD")]
    Grd,
    /// Greenland
    #[serde(rename = "GRL")]
    Grl,
    /// Republic of Guatemala
    #[serde(rename = "GTM")]
    Gtm,
    /// Territorial Collectivity of Guiana
    #[serde(rename = "GUF")]
    Guf,
    /// Territory of Guam
    #[serde(rename = "GUM")]
    Gum,
    /// Co-operative Republic of Guyana
    #[serde(rename = "GUY")]
    Guy,
    /// Hong Kong Special Administrative Region
    #[serde(rename = "HKG")]
    Hkg,
    /// Territory of Heard Island and McDonald Islands
    #[serde(rename = "HMD")]
    Hmd,
    /// Republic of Honduras
    #[serde(rename = "HND")]
    Hnd,
    /// Republic of Croatia
    #[serde(rename = "HRV")]
    Hrv,
    /// Republic of Haiti
    #[serde(rename = "HTI")]
    Hti,
    /// Hungary
    #[serde(rename = "HUN")]
    Hun,
    /// Republic of Indonesia
    #[serde(rename = "IDN")]
    Idn,
    /// Isle of Man
    #[serde(rename = "IMN")]
    Imn,
    /// Republic of India
    #[serde(rename = "IND")]
    Ind,
    /// British Indian Ocean Territory
    #[serde(rename = "IOT")]
    Iot,
    /// Ireland
    #[serde(rename = "IRL")]
    Irl,
    /// Islamic Republic of Iran
    #[serde(rename = "IRN")]
    Irn,
    /// Republic of Iraq
    #[serde(rename = "IRQ")]
    Irq,
    /// Republic of Iceland
    #[serde(rename = "ISL")]
    Isl,
    /// State of Israel
    #[serde(rename = "ISR")]
    Isr,
    /// Italian Republic
    #[serde(rename = "ITA")]
    Ita,
    /// Jamaica
    #[serde(rename = "JAM")]
    Jam,
    /// Bailiwick of Jersey
    #[serde(rename = "JEY")]
    Jey,
    /// Hashemite Kingdom of Jordan
    #[serde(rename = "JOR")]
    Jor,
    /// Japan
    #[serde(rename = "JPN")]
    Jpn,
    /// Republic of Kazakhstan
    #[serde(rename = "KAZ")]
    Kaz,
    /// Republic of Kenya
    #[serde(rename = "KEN")]
    Ken,
    /// Kyrgyz Republic
    #[serde(rename = "KGZ")]
    Kgz,
    /// Kingdom of Cambodia
    #[serde(rename = "KHM")]
    Khm,
    /// Republic of Kiribati
    #[serde(rename = "KIR")]
    Kir,
    /// Federation of Saint Kitts and Nevis
    #[serde(rename = "KNA")]
    Kna,
    /// Republic of Korea
    #[serde(rename = "KOR")]
    Kor,
    /// State of Kuwait
    #[serde(rename = "KWT")]
    Kwt,
    /// Lao People's Democratic Republic
    #[serde(rename = "LAO")]
    Lao,
    /// Lebanese Republic
    #[serde(rename = "LBN")]
    Lbn,
    /// Republic of Liberia
    #[serde(rename = "LBR")]
    Lbr,
    /// State of Libya
    #[serde(rename = "LBY")]
    Lby,
    /// Saint Lucia
    #[serde(rename = "LCA")]
    Lca,
    /// Principality of Liechtenstein
    #[serde(rename = "LIE")]
    Lie,
    /// Democratic Socialist Republic of Sri Lanka
    #[serde(rename = "LKA")]
    Lka,
    /// Kingdom of Lesotho
    #[serde(rename = "LSO")]
    Lso,
    /// Republic of Lithuania
    #[serde(rename = "LTU")]
    Ltu,
    /// Grand Duchy of Luxembourg
    #[serde(rename = "LUX")]
    Lux,
    /// Republic of Latvia
    #[serde(rename = "LVA")]
    Lva,
    /// Macau Special Administrative Region
    #[serde(rename = "MAC")]
    Mac,
    /// Saint Martin
    #[serde(rename = "MAF")]
    Maf,
    /// Kingdom of Morocco
    #[serde(rename = "MAR")]
    Mar,
    /// Principality of Monaco
    #[serde(rename = "MCO")]
    Mco,
    /// Republic of Moldova
    #[serde(rename = "MDA")]
    Mda,
    /// Republic of Madagascar
    #[serde(rename = "MDG")]
    Mdg,
    /// Republic of Maldives
    #[serde(rename = "MDV")]
    Mdv,
    /// United Mexican States
    #[serde(rename = "MEX")]
    Mex,
    /// Republic of the Marshall Islands
    #[serde(rename = "MHL")]
    Mhl,
    /// Republic of North Macedonia
    #[serde(rename = "MKD")]
    Mkd,
    /// Republic of Mali
    #[serde(rename = "MLI")]
    Mli,
    /// Republic of Malta
    #[serde(rename = "MLT")]
    Mlt,
    /// Union of Burma
    #[serde(rename = "MMR")]
    Mmr,
    /// Montenegro
    #[serde(rename = "MNE")]
    Mne,
    /// Mongolia
    #[serde(rename = "MNG")]
    Mng,
    /// Commonwealth of the Northern Mariana Islands
    #[serde(rename = "MNP")]
    Mnp,
    /// Republic of Mozambique
    #[serde(rename = "MOZ")]
    Moz,
    /// Islamic Republic of Mauritania
    #[serde(rename = "MRT")]
    Mrt,
    /// Montserrat
    #[serde(rename = "MSR")]
    Msr,
    /// Territorial Collectivity of Martinique
    #[serde(rename = "MTQ")]
    Mtq,
    /// Republic of Mauritius
    #[serde(rename = "MUS")]
    Mus,
    /// Republic of Malawi
    #[serde(rename = "MWI")]
    Mwi,
    /// Malaysia
    #[serde(rename = "MYS")]
    Mys,
    /// Department of Mayotte
    #[serde(rename = "MYT")]
    Myt,
    /// Republic of Namibia
    #[serde(rename = "NAM")]
    Nam,
    /// New Caledonia
    #[serde(rename = "NCL")]
    Ncl,
    /// Republic of Niger
    #[serde(rename = "NER")]
    Ner,
    /// Territory of Norfolk Island
    #[serde(rename = "NFK")]
    Nfk,
    /// Federal Republic of Nigeria
    #[serde(rename = "NGA")]
    Nga,
    /// Republic of Nicaragua
    #[serde(rename = "NIC")]
    Nic,
    /// Niue
    #[serde(rename = "NIU")]
    Niu,
    /// Kingdom of the Netherlands
    #[serde(rename = "NLD")]
    Nld,
    /// Kingdom of Norway
    #[serde(rename = "NOR")]
    Nor,
    /// Federal Democratic Republic of Nepal
    #[serde(rename = "NPL")]
    Npl,
    /// Republic of Nauru
    #[serde(rename = "NRU")]
    Nru,
    /// New Zealand
    #[serde(rename = "NZL")]
    Nzl,
    /// Sultanate of Oman
    #[serde(rename = "OMN")]
    Omn,
    /// Islamic Republic of Pakistan
    #[serde(rename = "PAK")]
    Pak,
    /// Republic of Panama
    #[serde(rename = "PAN")]
    Pan,
    /// Pitcairn, Henderson, Ducie, and Oeno Islands
    #[serde(rename = "PCN")]
    Pcn,
    /// Republic of Peru
    #[serde(rename = "PER")]
    Per,
    /// Republic of the Philippines
    #[serde(rename = "PHL")]
    Phl,
    /// Republic of Palau
    #[serde(rename = "PLW")]
    Plw,
    /// Independent State of Papua New Guinea
    #[serde(rename = "PNG")]
    Png,
    /// Republic of Poland
    #[serde(rename = "POL")]
    Pol,
    /// Commonwealth of Puerto Rico
    #[serde(rename = "PRI")]
    Pri,
    /// Democratic People's Republic of Korea
    #[serde(rename = "PRK")]
    Prk,
    /// Portuguese Republic
    #[serde(rename = "PRT")]
    Prt,
    /// Republic of Paraguay
    #[serde(rename = "PRY")]
    Pry,
    /// French Polynesia
    #[serde(rename = "PYF")]
    Pyf,
    /// State of Qatar
    #[serde(rename = "QAT")]
    Qat,
    /// Region of Reunion
    #[serde(rename = "REU")]
    Reu,
    /// Romania
    #[serde(rename = "ROU")]
    Rou,
    /// Russian Federation
    #[serde(rename = "RUS")]
    Rus,
    /// Republic of Rwanda
    #[serde(rename = "RWA")]
    Rwa,
    /// Kingdom of Saudi Arabia
    #[serde(rename = "SAU")]
    Sau,
    /// Republic of the Sudan
    #[serde(rename = "SDN")]
    Sdn,
    /// Republic of Senegal
    #[serde(rename = "SEN")]
    Sen,
    /// Republic of Singapore
    #[serde(rename = "SGP")]
    Sgp,
    /// South Georgia and the South Sandwich Islands
    #[serde(rename = "SGS")]
    Sgs,
    /// Saint Helena, Ascension, and Tristan da Cunha
    #[serde(rename = "SHN")]
    Shn,
    /// Solomon Islands
    #[serde(rename = "SLB")]
    Slb,
    /// Republic of Sierra Leone
    #[serde(rename = "SLE")]
    Sle,
    /// Republic of El Salvador
    #[serde(rename = "SLV")]
    Slv,
    /// Republic of San Marino
    #[serde(rename = "SMR")]
    Smr,
    /// Federal Republic of Somalia
    #[serde(rename = "SOM")]
    Som,
    /// Territorial Collectivity of Saint Pierre and Miquelon
    #[serde(rename = "SPM")]
    Spm,
    /// Republic of Serbia
    #[serde(rename = "SRB")]
    Srb,
    /// Republic of South Sudan
    #[serde(rename = "SSD")]
    Ssd,
    /// Democratic Republic of Sao Tome and Principe
    #[serde(rename = "STP")]
    Stp,
    /// Republic of Suriname
    #[serde(rename = "SUR")]
    Sur,
    /// Slovak Republic
    #[serde(rename = "SVK")]
    Svk,
    /// Republic of Slovenia
    #[serde(rename = "SVN")]
    Svn,
    /// Kingdom of Sweden
    #[serde(rename = "SWE")]
    Swe,
    /// Kingdom of Eswatini
    #[serde(rename = "SWZ")]
    Swz,
    /// Sint Maarten
    #[serde(rename = "SXM")]
    Sxm,
    /// Republic of Seychelles
    #[serde(rename = "SYC")]
    Syc,
    /// Syrian Arab Republic
    #[serde(rename = "SYR")]
    Syr,
    /// Turks and Caicos Islands
    #[serde(rename = "TCA")]
    Tca,
    /// Republic of Chad
    #[serde(rename = "TCD")]
    Tcd,
    /// Togolese Republic
    #[serde(rename = "TGO")]
    Tgo,
    /// Kingdom of Thailand
    #[serde(rename = "THA")]
    Tha,
    /// Republic of Tajikistan
    #[serde(rename = "TJK")]
    Tjk,
    /// Tokelau
    #[serde(rename = "TKL")]
    Tkl,
    /// Turkmenistan
    #[serde(rename = "TKM")]
    Tkm,
    /// Democratic Republic of Timor-Leste
    #[serde(rename = "TLS")]
    Tls,
    /// Kingdom of Tonga
    #[serde(rename = "TON")]
    Ton,
    /// Republic of Trinidad and Tobago
    #[serde(rename = "TTO")]
    Tto,
    /// Republic of Tunisia
    #[serde(rename = "TUN")]
    Tun,
    /// Republic of Turkey
    #[serde(rename = "TUR")]
    Tur,
    /// Tuvalu
    #[serde(rename = "TUV")]
    Tuv,
    /// Taiwan
    #[serde(rename = "TWN")]
    Twn,
    /// United Republic of Tanzania
    #[serde(rename = "TZA")]
    Tza,
    /// Republic of Uganda
    #[serde(rename = "UGA")]
    Uga,
    /// Ukraine
    #[serde(rename = "UKR")]
    Ukr,
    /// Oriental Republic of Uruguay
    #[serde(rename = "URY")]
    Ury,
    /// United States of America
    #[serde(rename = "USA")]
    Usa,
    /// Republic of Uzbekistan
    #[serde(rename = "UZB")]
    Uzb,
    /// State of the Vatican City
    #[serde(rename = "VAT")]
    Vat,
    /// Saint Vincent and the Grenadines
    #[serde(rename = "VCT")]
    Vct,
    /// Bolivarian Republic of Venezuela
    #[serde(rename = "VEN")]
    Ven,
    /// British Virgin Islands
    #[serde(rename = "VGB")]
    Vgb,
    /// United States Virgin Islands
    #[serde(rename = "VIR")]
    Vir,
    /// Socialist Republic of Vietnam
    #[serde(rename = "VNM")]
    Vnm,
    /// Republic of Vanuatu
    #[serde(rename = "VUT")]
    Vut,
    /// Wallis and Futuna
    #[serde(rename = "WLF")]
    Wlf,
    /// Independent State of Samoa
    #[serde(rename = "WSM")]
    Wsm,
    /// Territory of Ashmore and Cartier Islands
    #[serde(rename = "XAC")]
    Xac,
    /// Entity 1
    #[serde(rename = "XAZ")]
    Xaz,
    /// Bassas da India
    #[serde(rename = "XBI")]
    Xbi,
    /// Baker Island
    #[serde(rename = "XBK")]
    Xbk,
    /// Entity 2
    #[serde(rename = "XCR")]
    Xcr,
    /// Coral Sea Islands Territory
    #[serde(rename = "XCS")]
    Xcs,
    /// Entity 3
    #[serde(rename = "XCY")]
    Xcy,
    /// Europa Island
    #[serde(rename = "XEU")]
    Xeu,
    /// Glorioso Islands
    #[serde(rename = "XGL")]
    Xgl,
    /// Gaza Strip
    #[serde(rename = "XGZ")]
    Xgz,
    /// Howland Island
    #[serde(rename = "XHO")]
    Xho,
    /// Johnston Atoll
    #[serde(rename = "XJA")]
    Xja,
    /// Jan Mayen
    #[serde(rename = "XJM")]
    Xjm,
    /// Juan de Nova Island
    #[serde(rename = "XJN")]
    Xjn,
    /// Jarvis Island
    #[serde(rename = "XJV")]
    Xjv,
    /// Entity 4
    #[serde(rename = "XKM")]
    Xkm,
    /// Entity 5
    #[serde(rename = "XKN")]
    Xkn,
    /// Kingman Reef
    #[serde(rename = "XKR")]
    Xkr,
    /// Republic of Kosovo
    #[serde(rename = "XKS")]
    Xks,
    /// Midway Islands
    #[serde(rename = "XMW")]
    Xmw,
    /// Navassa Island
    #[serde(rename = "XNV")]
    Xnv,
    /// Palmyra Atoll
    #[serde(rename = "XPL")]
    Xpl,
    /// Paracel Islands
    #[serde(rename = "XPR")]
    Xpr,
    /// Akrotiri
    #[serde(rename = "XQZ")]
    Xqz,
    /// Spratly Islands
    #[serde(rename = "XSP")]
    Xsp,
    /// Svalbard
    #[serde(rename = "XSV")]
    Xsv,
    /// Tromelin Island
    #[serde(rename = "XTR")]
    Xtr,
    /// West Bank
    #[serde(rename = "XWB")]
    Xwb,
    /// Wake Island
    #[serde(rename = "XWK")]
    Xwk,
    /// Dhekelia
    #[serde(rename = "XXD")]
    Xxd,
    /// Republic of Yemen
    #[serde(rename = "YEM")]
    Yem,
    /// Republic of South Africa
    #[serde(rename = "ZAF")]
    Zaf,
    /// Republic of Zambia
    #[serde(rename = "ZMB")]
    Zmb,
    /// Republic of Zimbabwe
    #[serde(rename = "ZWE")]
    Zwe,
    /// FOUR EYES
    #[serde(rename = "ACGU")]
    Acgu,
    /// AFRICOM Multinational Strategic Partners
    #[serde(rename = "AMSP")]
    Amsp,
    /// Athens Olympics Security Coalition
    #[serde(rename = "AOSC")]
    Aosc,
    /// African Peacekeeping Force Somalia
    #[serde(rename = "APFS")]
    Apfs,
    /// Association of Southeast Asian Nations (ASEAN)
    #[serde(rename = "ASEA")]
    Asea,
    /// Australia Group
    #[serde(rename = "AUSTRALIA_GROUP")]
    AustraliaGroup,
    /// Boko Haram Task Force
    #[serde(rename = "BHTF")]
    Bhtf,
    /// Biological Weapons Convention States
    #[serde(rename = "BWCS")]
    Bwcs,
    /// Combined Forces Command Korea
    #[serde(rename = "CFCK")]
    Cfck,
    /// Coalition Forces Odyssey Dawn
    #[serde(rename = "CFOD")]
    Cfod,
    /// Coalition Forces Unified Protector
    #[serde(rename = "CFUP")]
    Cfup,
    /// Combined Libya Fusion Cell
    #[serde(rename = "CLFC")]
    Clfc,
    /// Combined Maritime Forces Central
    #[serde(rename = "CMFC")]
    Cmfc,
    /// Cooperative Maritime Forces Pacific
    #[serde(rename = "CMFP")]
    Cmfp,
    /// Civilian Protection Monitoring Team for Sudan
    #[serde(rename = "CPMT")]
    Cpmt,
    /// Countering Transnational Organized Crime
    #[serde(rename = "CTOC")]
    Ctoc,
    /// Chemical Weapons Convention States
    #[serde(rename = "CWCS")]
    Cwcs,
    /// European Counter-Terrorism Forces
    #[serde(rename = "ECTF")]
    Ectf,
    /// European Union Stabilization Forces in Bosnia
    #[serde(rename = "EFOR")]
    Efor,
    /// European Union
    #[serde(rename = "EU")]
    Eu,
    /// European Union DARFUR
    #[serde(rename = "EUDA")]
    Euda,
    /// Counter Violent Extremist Organizations Framework Partners
    #[serde(rename = "FRME")]
    Frme,
    /// FIVE EYES
    #[serde(rename = "FVEY")]
    Fvey,
    /// Gulf Cooperation Council
    #[serde(rename = "GCCH")]
    Gcch,
    /// Global Counter-Terrorism Forces
    #[serde(rename = "GCTF")]
    Gctf,
    /// Global Foreign Terrorist Fighter Network Exchange
    #[serde(rename = "GFNX")]
    Gfnx,
    /// Global Maritime Interception Forces
    #[serde(rename = "GMIF")]
    Gmif,
    /// International Events Security Coalition
    #[serde(rename = "IESC")]
    Iesc,
    /// International Maritime Security Construct
    #[serde(rename = "IMSC")]
    Imsc,
    /// INDOPACOM Multinational Strategic Partners
    #[serde(rename = "IMSP")]
    Imsp,
    /// INDO PACIFIC Maritime Call
    #[serde(rename = "IPMC")]
    Ipmc,
    /// Inherent Resolve Kinetic Support
    #[serde(rename = "IRKS")]
    Irks,
    /// International Security Assistance Force for Afghanistan
    #[serde(rename = "ISAF")]
    Isaf,
    /// International Syria Support Group
    #[serde(rename = "ISSG")]
    Issg,
    /// Stabilization Forces in Kosovo
    #[serde(rename = "KFOR")]
    Kfor,
    /// Multinational Coalition Forces-Iraq
    #[serde(rename = "MCFI")]
    Mcfi,
    /// Middle East Stability Force
    #[serde(rename = "MESF")]
    Mesf,
    /// Multinational GEOINT Europe
    #[serde(rename = "MGEU")]
    Mgeu,
    /// Multinational Interim Force Haiti
    #[serde(rename = "MIFH")]
    Mifh,
    /// Multi-Lateral Enduring Contingency
    #[serde(rename = "MLEC")]
    Mlec,
    /// Multinational Task Force
    #[serde(rename = "MNTF")]
    Mntf,
    /// Multinational Peacekeeping Forces
    #[serde(rename = "MPFL")]
    Mpfl,
    /// North African Counter-Terrorism Forces
    #[serde(rename = "NACT")]
    Nact,
    /// North Atlantic Treaty Organization
    #[serde(rename = "NATO")]
    Nato,
    /// NATO CFE Treaty on Conventional Armed Forces in Europe
    #[serde(rename = "NCFE")]
    Ncfe,
    /// North Korea Intelligence Coalition
    #[serde(rename = "NKIC")]
    Nkic,
    /// NORDIC
    #[serde(rename = "NRDC")]
    Nrdc,
    /// Nuclear Suppliers' Group
    #[serde(rename = "NSG")]
    Nsg,
    /// Olympic Security Advisory Group
    #[serde(rename = "OSAG")]
    Osag,
    /// Open Skies Treaty
    #[serde(rename = "OSTY")]
    Osty,
    /// Partnership for Actions in West Africa
    #[serde(rename = "PAWA")]
    Pawa,
    /// Persian Gulf Multinational Forces
    #[serde(rename = "PGMF")]
    Pgmf,
    /// Pacific Security Monitoring Exchange
    #[serde(rename = "PSMX")]
    Psmx,
    /// Russia Intelligence Sharing Coalition
    #[serde(rename = "RISC")]
    Risc,
    /// Resolute Support Mission Afghanistan
    #[serde(rename = "RSMA")]
    Rsma,
    /// Stabilization Force
    #[serde(rename = "SFOR")]
    Sfor,
    /// Special Operations Forces Partners
    #[serde(rename = "SOFP")]
    Sofp,
    /// SOF Planning Activities in Afghanistan (also called "11-Eyes")
    #[serde(rename = "SPAA")]
    Spaa,
    /// THREE EYES
    #[serde(rename = "TEYE")]
    Teye,
    /// Terrorist Financing Targeting Center
    #[serde(rename = "TFTC")]
    Tftc,
    /// United Nations Command, Korea
    #[serde(rename = "UNCK")]
    Unck,
}

/// CVEnumISMCATRelTo Values
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReleasableToEnum {
    /// United States of America
    #[serde(rename = "USA")]
    Usa,
    /// Aruba
    #[serde(rename = "ABW")]
    Abw,
    /// Islamic Republic of Afghanistan
    #[serde(rename = "AFG")]
    Afg,
    /// Republic of Angola
    #[serde(rename = "AGO")]
    Ago,
    /// Anguilla
    #[serde(rename = "AIA")]
    Aia,
    /// Republic of Albania
    #[serde(rename = "ALB")]
    Alb,
    /// Principality of Andorra
    #[serde(rename = "AND")]
    And,
    /// United Arab Emirates
    #[serde(rename = "ARE")]
    Are,
    /// Argentine Republic
    #[serde(rename = "ARG")]
    Arg,
    /// Republic of Armenia
    #[serde(rename = "ARM")]
    Arm,
    /// Territory of American Samoa
    #[serde(rename = "ASM")]
    Asm,
    /// Antarctica
    #[serde(rename = "ATA")]
    Ata,
    /// French Southern and Antarctic Lands
    #[serde(rename = "ATF")]
    Atf,
    /// Antigua and Barbuda
    #[serde(rename = "ATG")]
    Atg,
    /// Commonwealth of Australia
    #[serde(rename = "AUS")]
    Aus,
    /// Republic of Austria
    #[serde(rename = "AUT")]
    Aut,
    /// Guantanamo Bay Naval Base
    #[serde(rename = "AX2")]
    Ax2,
    /// Entity 6
    #[serde(rename = "AX3")]
    Ax3,
    /// Republic of Azerbaijan
    #[serde(rename = "AZE")]
    Aze,
    /// Republic of Burundi
    #[serde(rename = "BDI")]
    Bdi,
    /// Kingdom of Belgium
    #[serde(rename = "BEL")]
    Bel,
    /// Republic of Benin
    #[serde(rename = "BEN")]
    Ben,
    /// Bonaire, Sint Eustatius, and Saba
    #[serde(rename = "BES")]
    Bes,
    /// Burkina Faso
    #[serde(rename = "BFA")]
    Bfa,
    /// People's Republic of Bangladesh
    #[serde(rename = "BGD")]
    Bgd,
    /// Republic of Bulgaria
    #[serde(rename = "BGR")]
    Bgr,
    /// Kingdom of Bahrain
    #[serde(rename = "BHR")]
    Bhr,
    /// Commonwealth of The Bahamas
    #[serde(rename = "BHS")]
    Bhs,
    /// Bosnia and Herzegovina
    #[serde(rename = "BIH")]
    Bih,
    /// Saint Barthelemy
    #[serde(rename = "BLM")]
    Blm,
    /// Republic of Belarus
    #[serde(rename = "BLR")]
    Blr,
    /// Belize
    #[serde(rename = "BLZ")]
    Blz,
    /// Bermuda
    #[serde(rename = "BMU")]
    Bmu,
    /// Plurinational State of Bolivia
    #[serde(rename = "BOL")]
    Bol,
    /// Federative Republic of Brazil
    #[serde(rename = "BRA")]
    Bra,
    /// Barbados
    #[serde(rename = "BRB")]
    Brb,
    /// Brunei Darussalam
    #[serde(rename = "BRN")]
    Brn,
    /// Kingdom of Bhutan
    #[serde(rename = "BTN")]
    Btn,
    /// Bouvet Island
    #[serde(rename = "BVT")]
    Bvt,
    /// Republic of Botswana
    #[serde(rename = "BWA")]
    Bwa,
    /// Central African Republic
    #[serde(rename = "CAF")]
    Caf,
    /// Canada
    #[serde(rename = "CAN")]
    Can,
    /// Territory of Cocos (Keeling) Islands
    #[serde(rename = "CCK")]
    Cck,
    /// Swiss Confederation
    #[serde(rename = "CHE")]
    Che,
    /// Republic of Chile
    #[serde(rename = "CHL")]
    Chl,
    /// People's Republic of China
    #[serde(rename = "CHN")]
    Chn,
    /// Republic of Côte d'Ivoire
    #[serde(rename = "CIV")]
    Civ,
    /// Republic of Cameroon
    #[serde(rename = "CMR")]
    Cmr,
    /// Democratic Republic of the Congo
    #[serde(rename = "COD")]
    Cod,
    /// Republic of the Congo
    #[serde(rename = "COG")]
    Cog,
    /// Cook Islands
    #[serde(rename = "COK")]
    Cok,
    /// Republic of Colombia
    #[serde(rename = "COL")]
    Col,
    /// Union of the Comoros
    #[serde(rename = "COM")]
    Com,
    /// Clipperton Island
    #[serde(rename = "CPT")]
    Cpt,
    /// Republic of Cabo Verde
    #[serde(rename = "CPV")]
    Cpv,
    /// Republic of Costa Rica
    #[serde(rename = "CRI")]
    Cri,
    /// Republic of Cuba
    #[serde(rename = "CUB")]
    Cub,
    /// Curaçao
    #[serde(rename = "CUW")]
    Cuw,
    /// Territory of Christmas Island
    #[serde(rename = "CXR")]
    Cxr,
    /// Cayman Islands
    #[serde(rename = "CYM")]
    Cym,
    /// Republic of Cyprus
    #[serde(rename = "CYP")]
    Cyp,
    /// Czech Republic
    #[serde(rename = "CZE")]
    Cze,
    /// Federal Republic of Germany
    #[serde(rename = "DEU")]
    Deu,
    /// Diego Garcia
    #[serde(rename = "DGA")]
    Dga,
    /// Republic of Djibouti
    #[serde(rename = "DJI")]
    Dji,
    /// Commonwealth of Dominica
    #[serde(rename = "DMA")]
    Dma,
    /// Kingdom of Denmark
    #[serde(rename = "DNK")]
    Dnk,
    /// Dominican Republic
    #[serde(rename = "DOM")]
    Dom,
    /// People's Democratic Republic of Algeria
    #[serde(rename = "DZA")]
    Dza,
    /// Republic of Ecuador
    #[serde(rename = "ECU")]
    Ecu,
    /// Arab Republic of Egypt
    #[serde(rename = "EGY")]
    Egy,
    /// State of Eritrea
    #[serde(rename = "ERI")]
    Eri,
    /// Western Sahara
    #[serde(rename = "ESH")]
    Esh,
    /// Kingdom of Spain
    #[serde(rename = "ESP")]
    Esp,
    /// Republic of Estonia
    #[serde(rename = "EST")]
    Est,
    /// Federal Democratic Republic of Ethiopia
    #[serde(rename = "ETH")]
    Eth,
    /// Republic of Finland
    #[serde(rename = "FIN")]
    Fin,
    /// Republic of Fiji
    #[serde(rename = "FJI")]
    Fji,
    /// Falkland Islands (Islas Malvinas)
    #[serde(rename = "FLK")]
    Flk,
    /// French Republic
    #[serde(rename = "FRA")]
    Fra,
    /// Faroe Islands
    #[serde(rename = "FRO")]
    Fro,
    /// Federated States of Micronesia
    #[serde(rename = "FSM")]
    Fsm,
    /// Gabonese Republic
    #[serde(rename = "GAB")]
    Gab,
    /// United Kingdom of Great Britain and Northern Ireland
    #[serde(rename = "GBR")]
    Gbr,
    /// Georgia
    #[serde(rename = "GEO")]
    Geo,
    /// Bailiwick of Guernsey
    #[serde(rename = "GGY")]
    Ggy,
    /// Republic of Ghana
    #[serde(rename = "GHA")]
    Gha,
    /// Gibraltar
    #[serde(rename = "GIB")]
    Gib,
    /// Republic of Guinea
    #[serde(rename = "GIN")]
    Gin,
    /// Region of Guadeloupe
    #[serde(rename = "GLP")]
    Glp,
    /// Republic of The Gambia
    #[serde(rename = "GMB")]
    Gmb,
    /// Republic of Guinea-Bissau
    #[serde(rename = "GNB")]
    Gnb,
    /// Republic of Equatorial Guinea
    #[serde(rename = "GNQ")]
    Gnq,
    /// Hellenic Republic
    #[serde(rename = "GRC")]
    Grc,
    /// Grenada
    #[serde(rename = "GRD")]
    Grd,
    /// Greenland
    #[serde(rename = "GRL")]
    Grl,
    /// Republic of Guatemala
    #[serde(rename = "GTM")]
    Gtm,
    /// Territorial Collectivity of Guiana
    #[serde(rename = "GUF")]
    Guf,
    /// Territory of Guam
    #[serde(rename = "GUM")]
    Gum,
    /// Co-operative Republic of Guyana
    #[serde(rename = "GUY")]
    Guy,
    /// Hong Kong Special Administrative Region
    #[serde(rename = "HKG")]
    Hkg,
    /// Territory of Heard Island and McDonald Islands
    #[serde(rename = "HMD")]
    Hmd,
    /// Republic of Honduras
    #[serde(rename = "HND")]
    Hnd,
    /// Republic of Croatia
    #[serde(rename = "HRV")]
    Hrv,
    /// Republic of Haiti
    #[serde(rename = "HTI")]
    Hti,
    /// Hungary
    #[serde(rename = "HUN")]
    Hun,
    /// Republic of Indonesia
    #[serde(rename = "IDN")]
    Idn,
    /// Isle of Man
    #[serde(rename = "IMN")]
    Imn,
    /// Republic of India
    #[serde(rename = "IND")]
    Ind,
    /// British Indian Ocean Territory
    #[serde(rename = "IOT")]
    Iot,
    /// Ireland
    #[serde(rename = "IRL")]
    Irl,
    /// Islamic Republic of Iran
    #[serde(rename = "IRN")]
    Irn,
    /// Republic of Iraq
    #[serde(rename = "IRQ")]
    Irq,
    /// Republic of Iceland
    #[serde(rename = "ISL")]
    Isl,
    /// State of Israel
    #[serde(rename = "ISR")]
    Isr,
    /// Italian Republic
    #[serde(rename = "ITA")]
    Ita,
    /// Jamaica
    #[serde(rename = "JAM")]
    Jam,
    /// Bailiwick of Jersey
    #[serde(rename = "JEY")]
    Jey,
    /// Hashemite Kingdom of Jordan
    #[serde(rename = "JOR")]
    Jor,
    /// Japan
    #[serde(rename = "JPN")]
    Jpn,
    /// Republic of Kazakhstan
    #[serde(rename = "KAZ")]
    Kaz,
    /// Republic of Kenya
    #[serde(rename = "KEN")]
    Ken,
    /// Kyrgyz Republic
    #[serde(rename = "KGZ")]
    Kgz,
    /// Kingdom of Cambodia
    #[serde(rename = "KHM")]
    Khm,
    /// Republic of Kiribati
    #[serde(rename = "KIR")]
    Kir,
    /// Federation of Saint Kitts and Nevis
    #[serde(rename = "KNA")]
    Kna,
    /// Republic of Korea
    #[serde(rename = "KOR")]
    Kor,
    /// State of Kuwait
    #[serde(rename = "KWT")]
    Kwt,
    /// Lao People's Democratic Republic
    #[serde(rename = "LAO")]
    Lao,
    /// Lebanese Republic
    #[serde(rename = "LBN")]
    Lbn,
    /// Republic of Liberia
    #[serde(rename = "LBR")]
    Lbr,
    /// State of Libya
    #[serde(rename = "LBY")]
    Lby,
    /// Saint Lucia
    #[serde(rename = "LCA")]
    Lca,
    /// Principality of Liechtenstein
    #[serde(rename = "LIE")]
    Lie,
    /// Democratic Socialist Republic of Sri Lanka
    #[serde(rename = "LKA")]
    Lka,
    /// Kingdom of Lesotho
    #[serde(rename = "LSO")]
    Lso,
    /// Republic of Lithuania
    #[serde(rename = "LTU")]
    Ltu,
    /// Grand Duchy of Luxembourg
    #[serde(rename = "LUX")]
    Lux,
    /// Republic of Latvia
    #[serde(rename = "LVA")]
    Lva,
    /// Macau Special Administrative Region
    #[serde(rename = "MAC")]
    Mac,
    /// Saint Martin
    #[serde(rename = "MAF")]
    Maf,
    /// Kingdom of Morocco
    #[serde(rename = "MAR")]
    Mar,
    /// Principality of Monaco
    #[serde(rename = "MCO")]
    Mco,
    /// Republic of Moldova
    #[serde(rename = "MDA")]
    Mda,
    /// Republic of Madagascar
    #[serde(rename = "MDG")]
    Mdg,
    /// Republic of Maldives
    #[serde(rename = "MDV")]
    Mdv,
    /// United Mexican States
    #[serde(rename = "MEX")]
    Mex,
    /// Republic of the Marshall Islands
    #[serde(rename = "MHL")]
    Mhl,
    /// Republic of North Macedonia
    #[serde(rename = "MKD")]
    Mkd,
    /// Republic of Mali
    #[serde(rename = "MLI")]
    Mli,
    /// Republic of Malta
    #[serde(rename = "MLT")]
    Mlt,
    /// Union of Burma
    #[serde(rename = "MMR")]
    Mmr,
    /// Montenegro
    #[serde(rename = "MNE")]
    Mne,
    /// Mongolia
    #[serde(rename = "MNG")]
    Mng,
    /// Commonwealth of the Northern Mariana Islands
    #[serde(rename = "MNP")]
    Mnp,
    /// Republic of Mozambique
    #[serde(rename = "MOZ")]
    Moz,
    /// Islamic Republic of Mauritania
    #[serde(rename = "MRT")]
    Mrt,
    /// Montserrat
    #[serde(rename = "MSR")]
    Msr,
    /// Territorial Collectivity of Martinique
    #[serde(rename = "MTQ")]
    Mtq,
    /// Republic of Mauritius
    #[serde(rename = "MUS")]
    Mus,
    /// Republic of Malawi
    #[serde(rename = "MWI")]
    Mwi,
    /// Malaysia
    #[serde(rename = "MYS")]
    Mys,
    /// Department of Mayotte
    #[serde(rename = "MYT")]
    Myt,
    /// Republic of Namibia
    #[serde(rename = "NAM")]
    Nam,
    /// New Caledonia
    #[serde(rename = "NCL")]
    Ncl,
    /// Republic of Niger
    #[serde(rename = "NER")]
    Ner,
    /// Territory of Norfolk Island
    #[serde(rename = "NFK")]
    Nfk,
    /// Federal Republic of Nigeria
    #[serde(rename = "NGA")]
    Nga,
    /// Republic of Nicaragua
    #[serde(rename = "NIC")]
    Nic,
    /// Niue
    #[serde(rename = "NIU")]
    Niu,
    /// Kingdom of the Netherlands
    #[serde(rename = "NLD")]
    Nld,
    /// Kingdom of Norway
    #[serde(rename = "NOR")]
    Nor,
    /// Federal Democratic Republic of Nepal
    #[serde(rename = "NPL")]
    Npl,
    /// Republic of Nauru
    #[serde(rename = "NRU")]
    Nru,
    /// New Zealand
    #[serde(rename = "NZL")]
    Nzl,
    /// Sultanate of Oman
    #[serde(rename = "OMN")]
    Omn,
    /// Islamic Republic of Pakistan
    #[serde(rename = "PAK")]
    Pak,
    /// Republic of Panama
    #[serde(rename = "PAN")]
    Pan,
    /// Pitcairn, Henderson, Ducie, and Oeno Islands
    #[serde(rename = "PCN")]
    Pcn,
    /// Republic of Peru
    #[serde(rename = "PER")]
    Per,
    /// Republic of the Philippines
    #[serde(rename = "PHL")]
    Phl,
    /// Republic of Palau
    #[serde(rename = "PLW")]
    Plw,
    /// Independent State of Papua New Guinea
    #[serde(rename = "PNG")]
    Png,
    /// Republic of Poland
    #[serde(rename = "POL")]
    Pol,
    /// Commonwealth of Puerto Rico
    #[serde(rename = "PRI")]
    Pri,
    /// Democratic People's Republic of Korea
    #[serde(rename = "PRK")]
    Prk,
    /// Portuguese Republic
    #[serde(rename = "PRT")]
    Prt,
    /// Republic of Paraguay
    #[serde(rename = "PRY")]
    Pry,
    /// French Polynesia
    #[serde(rename = "PYF")]
    Pyf,
    /// State of Qatar
    #[serde(rename = "QAT")]
    Qat,
    /// Region of Reunion
    #[serde(rename = "REU")]
    Reu,
    /// Romania
    #[serde(rename = "ROU")]
    Rou,
    /// Russian Federation
    #[serde(rename = "RUS")]
    Rus,
    /// Republic of Rwanda
    #[serde(rename = "RWA")]
    Rwa,
    /// Kingdom of Saudi Arabia
    #[serde(rename = "SAU")]
    Sau,
    /// Republic of the Sudan
    #[serde(rename = "SDN")]
    Sdn,
    /// Republic of Senegal
    #[serde(rename = "SEN")]
    Sen,
    /// Republic of Singapore
    #[serde(rename = "SGP")]
    Sgp,
    /// South Georgia and the South Sandwich Islands
    #[serde(rename = "SGS")]
    Sgs,
    /// Saint Helena, Ascension, and Tristan da Cunha
    #[serde(rename = "SHN")]
    Shn,
    /// Solomon Islands
    #[serde(rename = "SLB")]
    Slb,
    /// Republic of Sierra Leone
    #[serde(rename = "SLE")]
    Sle,
    /// Republic of El Salvador
    #[serde(rename = "SLV")]
    Slv,
    /// Republic of San Marino
    #[serde(rename = "SMR")]
    Smr,
    /// Federal Republic of Somalia
    #[serde(rename = "SOM")]
    Som,
    /// Territorial Collectivity of Saint Pierre and Miquelon
    #[serde(rename = "SPM")]
    Spm,
    /// Republic of Serbia
    #[serde(rename = "SRB")]
    Srb,
    /// Republic of South Sudan
    #[serde(rename = "SSD")]
    Ssd,
    /// Democratic Republic of Sao Tome and Principe
    #[serde(rename = "STP")]
    Stp,
    /// Republic of Suriname
    #[serde(rename = "SUR")]
    Sur,
    /// Slovak Republic
    #[serde(rename = "SVK")]
    Svk,
    /// Republic of Slovenia
    #[serde(rename = "SVN")]
    Svn,
    /// Kingdom of Sweden
    #[serde(rename = "SWE")]
    Swe,
    /// Kingdom of Eswatini
    #[serde(rename = "SWZ")]
    Swz,
    /// Sint Maarten
    #[serde(rename = "SXM")]
    Sxm,
    /// Republic of Seychelles
    #[serde(rename = "SYC")]
    Syc,
    /// Syrian Arab Republic
    #[serde(rename = "SYR")]
    Syr,
    /// Turks and Caicos Islands
    #[serde(rename = "TCA")]
    Tca,
    /// Republic of Chad
    #[serde(rename = "TCD")]
    Tcd,
    /// Togolese Republic
    #[serde(rename = "TGO")]
    Tgo,
    /// Kingdom of Thailand
    #[serde(rename = "THA")]
    Tha,
    /// Republic of Tajikistan
    #[serde(rename = "TJK")]
    Tjk,
    /// Tokelau
    #[serde(rename = "TKL")]
    Tkl,
    /// Turkmenistan
    #[serde(rename = "TKM")]
    Tkm,
    /// Democratic Republic of Timor-Leste
    #[serde(rename = "TLS")]
    Tls,
    /// Kingdom of Tonga
    #[serde(rename = "TON")]
    Ton,
    /// Republic of Trinidad and Tobago
    #[serde(rename = "TTO")]
    Tto,
    /// Republic of Tunisia
    #[serde(rename = "TUN")]
    Tun,
    /// Republic of Turkey
    #[serde(rename = "TUR")]
    Tur,
    /// Tuvalu
    #[serde(rename = "TUV")]
    Tuv,
    /// Taiwan
    #[serde(rename = "TWN")]
    Twn,
    /// United Republic of Tanzania
    #[serde(rename = "TZA")]
    Tza,
    /// Republic of Uganda
    #[serde(rename = "UGA")]
    Uga,
    /// Ukraine
    #[serde(rename = "UKR")]
    Ukr,
    /// Oriental Republic of Uruguay
    #[serde(rename = "URY")]
    Ury,
    /// Republic of Uzbekistan
    #[serde(rename = "UZB")]
    Uzb,
    /// State of the Vatican City
    #[serde(rename = "VAT")]
    Vat,
    /// Saint Vincent and the Grenadines
    #[serde(rename = "VCT")]
    Vct,
    /// Bolivarian Republic of Venezuela
    #[serde(rename = "VEN")]
    Ven,
    /// British Virgin Islands
    #[serde(rename = "VGB")]
    Vgb,
    /// United States Virgin Islands
    #[serde(rename = "VIR")]
    Vir,
    /// Socialist Republic of Vietnam
    #[serde(rename = "VNM")]
    Vnm,
    /// Republic of Vanuatu
    #[serde(rename = "VUT")]
    Vut,
    /// Wallis and Futuna
    #[serde(rename = "WLF")]
    Wlf,
    /// Independent State of Samoa
    #[serde(rename = "WSM")]
    Wsm,
    /// Territory of Ashmore and Cartier Islands
    #[serde(rename = "XAC")]
    Xac,
    /// Entity 1
    #[serde(rename = "XAZ")]
    Xaz,
    /// Bassas da India
    #[serde(rename = "XBI")]
    Xbi,
    /// Baker Island
    #[serde(rename = "XBK")]
    Xbk,
    /// Entity 2
    #[serde(rename = "XCR")]
    Xcr,
    /// Coral Sea Islands Territory
    #[serde(rename = "XCS")]
    Xcs,
    /// Entity 3
    #[serde(rename = "XCY")]
    Xcy,
    /// Europa Island
    #[serde(rename = "XEU")]
    Xeu,
    /// Glorioso Islands
    #[serde(rename = "XGL")]
    Xgl,
    /// Gaza Strip
    #[serde(rename = "XGZ")]
    Xgz,
    /// Howland Island
    #[serde(rename = "XHO")]
    Xho,
    /// Johnston Atoll
    #[serde(rename = "XJA")]
    Xja,
    /// Jan Mayen
    #[serde(rename = "XJM")]
    Xjm,
    /// Juan de Nova Island
    #[serde(rename = "XJN")]
    Xjn,
    /// Jarvis Island
    #[serde(rename = "XJV")]
    Xjv,
    /// Entity 4
    #[serde(rename = "XKM")]
    Xkm,
    /// Entity 5
    #[serde(rename = "XKN")]
    Xkn,
    /// Kingman Reef
    #[serde(rename = "XKR")]
    Xkr,
    /// Republic of Kosovo
    #[serde(rename = "XKS")]
    Xks,
    /// Midway Islands
    #[serde(rename = "XMW")]
    Xmw,
    /// Navassa Island
    #[serde(rename = "XNV")]
    Xnv,
    /// Palmyra Atoll
    #[serde(rename = "XPL")]
    Xpl,
    /// Paracel Islands
    #[serde(rename = "XPR")]
    Xpr,
    /// Akrotiri
    #[serde(rename = "XQZ")]
    Xqz,
    /// Spratly Islands
    #[serde(rename = "XSP")]
    Xsp,
    /// Svalbard
    #[serde(rename = "XSV")]
    Xsv,
    /// Tromelin Island
    #[serde(rename = "XTR")]
    Xtr,
    /// West Bank
    #[serde(rename = "XWB")]
    Xwb,
    /// Wake Island
    #[serde(rename = "XWK")]
    Xwk,
    /// Dhekelia
    #[serde(rename = "XXD")]
    Xxd,
    /// Republic of Yemen
    #[serde(rename = "YEM")]
    Yem,
    /// Republic of South Africa
    #[serde(rename = "ZAF")]
    Zaf,
    /// Republic of Zambia
    #[serde(rename = "ZMB")]
    Zmb,
    /// Republic of Zimbabwe
    #[serde(rename = "ZWE")]
    Zwe,
    /// FOUR EYES
    #[serde(rename = "ACGU")]
    Acgu,
    /// AFRICOM Multinational Strategic Partners
    #[serde(rename = "AMSP")]
    Amsp,
    /// Athens Olympics Security Coalition
    #[serde(rename = "AOSC")]
    Aosc,
    /// African Peacekeeping Force Somalia
    #[serde(rename = "APFS")]
    Apfs,
    /// Association of Southeast Asian Nations (ASEAN)
    #[serde(rename = "ASEA")]
    Asea,
    /// Australia Group
    #[serde(rename = "AUSTRALIA_GROUP")]
    AustraliaGroup,
    /// Boko Haram Task Force
    #[serde(rename = "BHTF")]
    Bhtf,
    /// Biological Weapons Convention States
    #[serde(rename = "BWCS")]
    Bwcs,
    /// Combined Forces Command Korea
    #[serde(rename = "CFCK")]
    Cfck,
    /// Coalition Forces Odyssey Dawn
    #[serde(rename = "CFOD")]
    Cfod,
    /// Coalition Forces Unified Protector
    #[serde(rename = "CFUP")]
    Cfup,
    /// Combined Libya Fusion Cell
    #[serde(rename = "CLFC")]
    Clfc,
    /// Combined Maritime Forces Central
    #[serde(rename = "CMFC")]
    Cmfc,
    /// Cooperative Maritime Forces Pacific
    #[serde(rename = "CMFP")]
    Cmfp,
    /// Civilian Protection Monitoring Team for Sudan
    #[serde(rename = "CPMT")]
    Cpmt,
    /// Countering Transnational Organized Crime
    #[serde(rename = "CTOC")]
    Ctoc,
    /// Chemical Weapons Convention States
    #[serde(rename = "CWCS")]
    Cwcs,
    /// European Counter-Terrorism Forces
    #[serde(rename = "ECTF")]
    Ectf,
    /// European Union Stabilization Forces in Bosnia
    #[serde(rename = "EFOR")]
    Efor,
    /// European Union
    #[serde(rename = "EU")]
    Eu,
    /// European Union DARFUR
    #[serde(rename = "EUDA")]
    Euda,
    /// Counter Violent Extremist Organizations Framework Partners
    #[serde(rename = "FRME")]
    Frme,
    /// FIVE EYES
    #[serde(rename = "FVEY")]
    Fvey,
    /// Gulf Cooperation Council
    #[serde(rename = "GCCH")]
    Gcch,
    /// Global Counter-Terrorism Forces
    #[serde(rename = "GCTF")]
    Gctf,
    /// Global Foreign Terrorist Fighter Network Exchange
    #[serde(rename = "GFNX")]
    Gfnx,
    /// Global Maritime Interception Forces
    #[serde(rename = "GMIF")]
    Gmif,
    /// International Events Security Coalition
    #[serde(rename = "IESC")]
    Iesc,
    /// International Maritime Security Construct
    #[serde(rename = "IMSC")]
    Imsc,
    /// INDOPACOM Multinational Strategic Partners
    #[serde(rename = "IMSP")]
    Imsp,
    /// INDO PACIFIC Maritime Call
    #[serde(rename = "IPMC")]
    Ipmc,
    /// Inherent Resolve Kinetic Support
    #[serde(rename = "IRKS")]
    Irks,
    /// International Security Assistance Force for Afghanistan
    #[serde(rename = "ISAF")]
    Isaf,
    /// International Syria Support Group
    #[serde(rename = "ISSG")]
    Issg,
    /// Stabilization Forces in Kosovo
    #[serde(rename = "KFOR")]
    Kfor,
    /// Multinational Coalition Forces-Iraq
    #[serde(rename = "MCFI")]
    Mcfi,
    /// Middle East Stability Force
    #[serde(rename = "MESF")]
    Mesf,
    /// Multinational GEOINT Europe
    #[serde(rename = "MGEU")]
    Mgeu,
    /// Multinational Interim Force Haiti
    #[serde(rename = "MIFH")]
    Mifh,
    /// Multi-Lateral Enduring Contingency
    #[serde(rename = "MLEC")]
    Mlec,
    /// Multinational Task Force
    #[serde(rename = "MNTF")]
    Mntf,
    /// Multinational Peacekeeping Forces
    #[serde(rename = "MPFL")]
    Mpfl,
    /// North African Counter-Terrorism Forces
    #[serde(rename = "NACT")]
    Nact,
    /// North Atlantic Treaty Organization
    #[serde(rename = "NATO")]
    Nato,
    /// NATO CFE Treaty on Conventional Armed Forces in Europe
    #[serde(rename = "NCFE")]
    Ncfe,
    /// North Korea Intelligence Coalition
    #[serde(rename = "NKIC")]
    Nkic,
    /// NORDIC
    #[serde(rename = "NRDC")]
    Nrdc,
    /// Nuclear Suppliers' Group
    #[serde(rename = "NSG")]
    Nsg,
    /// Olympic Security Advisory Group
    #[serde(rename = "OSAG")]
    Osag,
    /// Open Skies Treaty
    #[serde(rename = "OSTY")]
    Osty,
    /// Partnership for Actions in West Africa
    #[serde(rename = "PAWA")]
    Pawa,
    /// Persian Gulf Multinational Forces
    #[serde(rename = "PGMF")]
    Pgmf,
    /// Pacific Security Monitoring Exchange
    #[serde(rename = "PSMX")]
    Psmx,
    /// Russia Intelligence Sharing Coalition
    #[serde(rename = "RISC")]
    Risc,
    /// Resolute Support Mission Afghanistan
    #[serde(rename = "RSMA")]
    Rsma,
    /// Stabilization Force
    #[serde(rename = "SFOR")]
    Sfor,
    /// Special Operations Forces Partners
    #[serde(rename = "SOFP")]
    Sofp,
    /// SOF Planning Activities in Afghanistan (also called "11-Eyes")
    #[serde(rename = "SPAA")]
    Spaa,
    /// THREE EYES
    #[serde(rename = "TEYE")]
    Teye,
    /// Terrorist Financing Targeting Center
    #[serde(rename = "TFTC")]
    Tftc,
    /// United Nations Command, Korea
    #[serde(rename = "UNCK")]
    Unck,
}

/// CVEnumISMSCIControls Values
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SciControlsEnum {
    /// BUR
    #[serde(rename = "BUR")]
    Bur,
    /// BUR-BLG
    #[serde(rename = "BUR_BLG")]
    BurBlg,
    /// BUR-DTP
    #[serde(rename = "BUR_DTP")]
    BurDtp,
    /// BUR-WRG
    #[serde(rename = "BUR_WRG")]
    BurWrg,
    /// HCS
    #[serde(rename = "HCS")]
    Hcs,
    /// HCS-O
    #[serde(rename = "HCS_O")]
    HcsO,
    /// HCS-P
    #[serde(rename = "HCS_P")]
    HcsP,
    /// HCS-X
    #[serde(rename = "HCS_X")]
    HcsX,
    /// KLAMATH
    #[serde(rename = "KLM")]
    Klm,
    /// KLAMATH-R
    #[serde(rename = "KLM_R")]
    KlmR,
    /// MARVEL
    #[serde(rename = "MVL")]
    Mvl,
    /// RESERVE
    #[serde(rename = "RSV")]
    Rsv,
    /// SPECIAL INTELLIGENCE
    #[serde(rename = "SI")]
    Si,
    /// ECRU
    #[serde(rename = "SI_EU")]
    SiEu,
    /// SI-GAMMA
    #[serde(rename = "SI_G")]
    SiG,
    /// NONBOOK
    #[serde(rename = "SI_NK")]
    SiNk,
    /// TALENT KEYHOLE
    #[serde(rename = "TK")]
    Tk,
    /// BLUEFISH
    #[serde(rename = "TK_BLFH")]
    TkBlfh,
    /// IDITAROD
    #[serde(rename = "TK_IDIT")]
    TkIdit,
    /// KANDIK
    #[serde(rename = "TK_KAND")]
    TkKand,
}

/// (U) All currently valid markings that can appear in the second banner line. This enum is used by
/// SecondBannerLine. PERMISSIBLE VALUES The permissible values for this simple type are defined in
/// the Controlled Value Enumeration: CVEnumISMSecondBannerLine.xml
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecondBannerLineEnum {
    /// ATTORNEY-CLIENT PRIVILEGED INFO
    #[serde(rename = "ACPI")]
    Acpi,
    /// ATTORNEY WORK PRODUCT
    #[serde(rename = "AWP")]
    Awp,
    /// CONTENTS INCLUDE US PERSON INFORMATION
    #[serde(rename = "CUSPI")]
    Cuspi,
    /// DELIBERATIVE PROCESS PRIVILEGED DOCUMENT
    #[serde(rename = "DPPD")]
    Dppd,
    /// HANDLE VIA CHANNELS ONLY
    #[serde(rename = "HVCO")]
    Hvco,
    /// SOURCE SELECTION SENSITIVE
    #[serde(rename = "SSS")]
    Sss,
}

/// An enumeration specifying the state of a command sent to a service or service manager.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceLifecycleCommandStatusEnum {
    /// A status specifying the command sent to the service or service manager executed properly.
    #[serde(rename = "SUCCESS")]
    Success,
    /// A status specifying the command sent to the service or service manager failed to be executed
    /// properly.
    #[serde(rename = "FAIL")]
    Fail,
}

/// An enumeration of commands that can be sent to a service or service manager.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceLifecycleStateEnum {
    /// A command to start a service lifecycle in its INITIALIZING state. This command will likely be
    /// handled by a resource manager to startup a service. Services will not normally start itself.
    #[serde(rename = "START")]
    Start,
    /// A command to stop a service lifecycle by transitioning it into its INOPERABLE state. This
    /// command will likely be handled by a resource manager to stop a service.
    #[serde(rename = "STOP")]
    Stop,
    /// A command to restart a service lifecycle by transitioning it back into its INITIALIZING state
    /// from any state except the INOPERABLE state. This command will likely be handled by a resource
    /// manager to restart a service.
    #[serde(rename = "RESTART")]
    Restart,
    /// A command to reset a service lifecycle by resetting its internal parameters in any state except
    /// the INOPERABLE state. This may or may not cause the service to transition to another state. This
    /// command will likely be handled by a resource manager to reset a service. Note: RESTART (not
    /// RESET) is used to return a service to its INITIALIZING state.
    #[serde(rename = "RESET")]
    Reset,
    /// A command to temporarily pause a service lifecycle by transitioning the service into its PAUSED
    /// state where its current operations are suspended until a subsequent resume command is received.
    /// This command will be rejected if it's in its PAUSED state and it will be ignored if the service
    /// is in its INOPERABLE state. This command will likely be handled by a resource manager to pause a
    /// service.
    #[serde(rename = "PAUSE")]
    Pause,
    /// A command to resume operation of a paused service lifecycle by transitioning it into the
    /// previous state in which it received the pause command. This command will be rejected if it's not
    /// in its PAUSED state and it will be ignored if the service is in its INOPERABLE state. This
    /// command will likely be handled by a resource manager to resume a paused service.
    #[serde(rename = "RESUME")]
    Resume,
}

/// Indicates Service independent settings for any Service.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceSettingEnum {
    /// Indicates a setting in which the Service will distribute audit records per the defined
    /// behaviors. This setting is independent of other service settings.
    #[serde(rename = "AUDIT_LOGGING")]
    AuditLogging,
    /// Indicates a setting in which the Service operates with debug reporting outputs and functional
    /// behaviors. This setting is independent of other Service settings.
    #[serde(rename = "DEBUG_REPORTING")]
    DebugReporting,
}

/// Indicates the Status state machine for a Service.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceStateEnum {
    /// Indicates the Service is starting up. Communication has begun but Service startup is not
    /// complete (e.g. still loading configuration files).
    #[serde(rename = "INITIALIZING")]
    Initializing,
    /// Indicates the Service is functioning normally. Configuration files loaded. All functions can now
    /// be used.
    #[serde(rename = "NORMAL")]
    Normal,
    /// Indicates the Service is not functioning normally. Functions can be used, but there may be
    /// limitations. Some defined Service functions may be available but may not be relied upon due to
    /// degraded state of Service.
    #[serde(rename = "DEGRADED")]
    Degraded,
    /// Indicates the Service is functioning properly, but has been paused (e.g. by operator
    /// interaction, because of dependency).
    #[serde(rename = "PAUSED")]
    Paused,
    /// Indicates the Service is unusable.
    #[serde(rename = "INOPERABLE")]
    Inoperable,
}

/// String representing the UCI version.
pub type UciSchemaVersionStringType = String;

/// A UUID is a 128-bit number (32 hexadecimal digits, 16 bytes) that is conformant to any version
/// of variant 1 or nil UUID, as described in IETF RFC 4122.
pub type UniversallyUniqueIdentifierType = String;

/// A string representing up to 1024 characters in length, restricted to visible characters
/// (0x20-0x7E).
pub type VisibleString1024Type = String;

/// A string representing up to 256 characters in length, restricted to visible characters
/// (0x20-0x7E).
pub type VisibleString256Type = String;

/// A string representing up to 64 characters in length, restricted to visible characters
/// (0x20-0x7E).
pub type VisibleString64Type = String;

/// String type of 1024 characters in length, restricted to visible characters (0x21-0x7E) and
/// whitespace characters.
pub type WhitespaceVisibleString1024Type = String;

/// String type of 4096 characters in length, restricted to visible characters (0x21-0x7E) and
/// whitespace characters.
pub type WhitespaceVisibleString4096Type = String;

/// The ID type for UCI IDs that correspond to ACTDF Tasks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActdfTaskIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to AccessAssessments.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccessAssessmentIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Actions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActionIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// The ID type for UCI IDs that correspond to ActionPlanCommand messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActionPlanCommandIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to ActionPlan messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActionPlanIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// The ID type for UCI IDs that correspond to ActionPlanValidation messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActionPlanValidationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to *Capability Activities.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActivityIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Activity Metrics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActivityMetricsIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// The ID type for UCI IDs that correspond to Route plan commands.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActivityPlanCommandIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// This type is the standard UCI ID type that corresponds to an ActivityPlan.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActivityPlanIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// ID Type of the UCI IDs that correspond to the ActivityPlanValidation messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActivityPlanValidationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to AirCharacteristics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AirCharacteristicsIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Air Records.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AirRecordIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to platform AirReport messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AirReportIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// The ID type that refers to an airfield report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AirfieldIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to this AirfieldReport.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AirfieldReportIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The UUID for analysis routes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnalysisRouteIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AntennaIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// Unique ID type that identifies Antenna Resources.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AntennaResourceIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to approval authority.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApprovalAuthorityIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to approval policies.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApprovalPolicyIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to assessments.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AssessmentIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to authorizations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorizationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to autonomous actions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AutonomousPlanningActionIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BitIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to beams.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BeamIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CalibrationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// Indicates an associated call sign. Multiple call signs are allowed for different communications
/// systems, registries or other reasons. The use of ForeignKeyType is an indication that call signs
/// aren't universally defined and come from multiple systems/sources but they are expected to be
/// unique within the context of their source. For a call sign in UCI, the CallSignType.Key element
/// indicates the call sign itself and the CallSignType.SystemName element indicates the system that
/// assigned it and ensures it is unique. For example, a call sign that came from a Link-16 system
/// might have Key=Phantom01 and SystemName=Link16Voice.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CallSignType {
    /// This element contains the value of the ForeignKey.
    #[serde(rename = "Key")]
    pub key: VisibleString256Type,
    /// This element refers to the "system" where the ForeignKey is managed. That system could be
    /// another message standard/protocol, network, database, shared table, etc. UCI leaves it to
    /// implementers to choose and coordinate use of the foreign "system".
    #[serde(rename = "SystemName")]
    pub system_name: VisibleString256Type,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CannotComplyType {
    /// Indicates the reason why an action (Task, \[Capability\]Command,
    /// \[SupportingCapability\]Command, \[Capability\]Activity, etc.) was rejected, interrupted,
    /// unallocated, failed and/or generally can't be completed satisfactorily. See enumeration
    /// annotations for further details.
    #[serde(rename = "Reason")]
    pub reason: CannotComplyEnum,
    /// Indicates a human-readable text description of the reason for the "can't comply".
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<VisibleString1024Type>,
    /// Indicates an ID corresponding to another object that is associated with or the cause of the
    /// "can't comply".
    #[serde(rename = "AssociatedID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_id: Option<IdTypeValue>,
}

/// The ID type for UCI IDs that correspond to System Loadouts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityConfigurationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type that refers to a specific operator annotation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityCoverageAreaIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to capability coverage area processes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityCoverageAreaProcessIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type that refers to a commandable and/or autonomous Capability of a Subsystem.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to a cargo item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CargoIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommAntennaModeType {
    /// This element contains the value of the ForeignKey.
    #[serde(rename = "Key")]
    pub key: VisibleString256Type,
    /// This element refers to the "system" where the ForeignKey is managed. That system could be
    /// another message standard/protocol, network, database, shared table, etc. UCI leaves it to
    /// implementers to choose and coordinate use of the foreign "system".
    #[serde(rename = "SystemName")]
    pub system_name: VisibleString256Type,
}

/// Represents the ID of a configuration for a communication C2 adapter or Gateway.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommConfigurationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type that refers to an aggregation of effective communications configurations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommEffectivityIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Pointing Vector commands.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommPointingIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommProfileIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to external communication allocations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommSupportIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to CommSupportPlanCommand messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommSupportPlanCommandIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to TaskPlan messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommSupportPlanIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// The ID type for UCI IDs that correspond to CommSupportPlanValidation messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommSupportPlanValidationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to mission plans.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommTerminalPlanIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommUserIoIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommUserLinkIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to external communication allocations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommWindowIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The base type for all *Command messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommandBaseType {
    /// Indicates the unique ID of the Command corresponding to this message. Used to reference the
    /// command when providing status.
    #[serde(rename = "CommandID")]
    pub command_id: CommandIdType,
    /// The state of the command message data. Indicates if a command message is a NEW command, an
    /// UPDATE to an existing command, or a CANCEL of an existing command.
    #[serde(rename = "CommandState")]
    pub command_state: CommandStateEnum,
}

/// The ID type for UCI IDs that correspond to command messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommandIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The base type for all *CommandStatus messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommandStatusBaseType {
    /// Indicates the unique ID of the command corresponding to this status message.
    #[serde(rename = "CommandID")]
    pub command_id: CommandIdType,
    /// Indicates the state of the command. The command state machine assumes that the Command and
    /// CommandStatus messaging interaction is only used to communicate the desired Command to the
    /// Subsystem or Service. The state machine and messaging terminates as soon as the host accepts or
    /// rejects the Command. See enumeration annotations for further details.
    #[serde(rename = "CommandProcessingState")]
    pub command_processing_state: CommandProcessingStateEnum,
    /// Indicates details regarding the reason for and/or a contributing factor to the sibling
    /// CommandProcessingState element. This element is only expected when the Command is in a
    /// potentially undesirable state such as rejected, canceled or accepted with less than optimum
    /// results expected.
    #[serde(rename = "CommandProcessingStateReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_processing_state_reason: Option<CannotComplyTypeValue>,
}

/// The ID type for UCI IDs that correspond to a ComponentConfiguration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComponentConfigurationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID of an element or sub-component within a component.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComponentElementIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to resource capabilities.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComponentIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// Indicates the ConflictID UUID.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConflictIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to CoordinatedRequirementSet messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoordinatedRequirementSetIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// Indicates the unique ID of an Entity which is eligible for or excluded from future automated
/// fusion operations with the Entity represented by this Entity message. This element is required
/// when the sibling FusionEligibility element indicates SELECTIVE_EXCLUSION or SELECTIVE_INCLUSION
/// and should be omitted otherwise.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CorrelatedEntityIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Optional reference to entity type which is either eligible for or excluded from future automated
    /// fusion operations with the Entity represented by this Entity message.
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<EntitySourceEnum>,
}

/// An ID type for a CounterSpaceDeconfliction message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CounterSpaceDeconflictionIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// An ID type for a CounterSpaceReport.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CounterSpaceReportIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to crypto ports.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CryptoPortIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to crypto slot groups.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CryptoSlotGroupIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to DLZs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DlzIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to DMPI designations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DmpiDesignationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to DMPIs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DmpiIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// The ID type for UCI IDs that correspond to DMPI patterns.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DmpiPatternIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to damage assessments.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DamageAssessmentIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to damage estimates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DamageEstimateIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to damage information reports.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DamageReportIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to damage information summary reports.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DamageSummaryIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Gateway DataDestinations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DataDestinationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Data Plans.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DataPlanIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
    /// This element is used to trace a UCI Data plan to a foreign, non-UCI ID. This supports many use
    /// cases where there is requirement to adapt UCI Data plans to/from other formats. This includes
    /// conversion of a UCI Data plan to a System specific format, or conversion of a non-UCI Data plan
    /// to UCI. The inclusion of this data in the Data plan allows the ID mappings/associations to be
    /// managed/persisted natively in UCI and negates the need to persist the mappings in an out-of-band
    /// format.
    #[serde(rename = "ForeignDataPlanKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreign_data_plan_key: Option<ForeignKeyTypeValue>,
}

/// The ID type for UCI IDs that correspond to DataRecord instances.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DataRecordInstanceIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to DataRecordLists.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DataRecordListIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// Indicates a time range with beginning and ending dates and times. A range with a Begin and no
/// End is understood to continue indefinitely or until context implies an end. A range with an End
/// and no Begin is understood to start "now". A range with no Begin or End is understood to start
/// "now" and continue indefinitely or until context implies an end.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DateTimeRangeType {
    /// This element represents the absolute begin/start time and date for the time range.
    #[serde(rename = "Begin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<DateTimeType>,
    /// This element represents the absolute End time and date for the time range.
    #[serde(rename = "End")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTimeType>,
}

/// The ID type for UCI IDs that correspond to Designation messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DesignationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DwellIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// Indicates an ID corresponding to an EA technique. The use of ForeignKeyType is an indication
/// that EA techniques aren't universally defined and come from multiple systems/sources but they
/// are expected to be unique within the context of their source. For an EA technique in UCI, the
/// EA_TechniqueIdentifierType.Key element indicates the technique ID and the
/// EA_TechniqueIdentifierType.SystemName element indicates the system that assigned the ID and
/// ensures it is unique.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EaTechniqueIdentifierType {
    /// This element contains the value of the ForeignKey.
    #[serde(rename = "Key")]
    pub key: VisibleString256Type,
    /// This element refers to the "system" where the ForeignKey is managed. That system could be
    /// another message standard/protocol, network, database, shared table, etc. UCI leaves it to
    /// implementers to choose and coordinate use of the foreign "system".
    #[serde(rename = "SystemName")]
    pub system_name: VisibleString256Type,
}

/// The ID type for UCI IDs that correspond to EEI.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EeiIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EobEquipmentType {
    /// This element contains the value of the ForeignKey.
    #[serde(rename = "Key")]
    pub key: VisibleString256Type,
    /// This element refers to the "system" where the ForeignKey is managed. That system could be
    /// another message standard/protocol, network, database, shared table, etc. UCI leaves it to
    /// implementers to choose and coordinate use of the foreign "system".
    #[serde(rename = "SystemName")]
    pub system_name: VisibleString256Type,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EobFacilityPinType {
    /// This element contains the value of the ForeignKey.
    #[serde(rename = "Key")]
    pub key: VisibleString256Type,
    /// This element refers to the "system" where the ForeignKey is managed. That system could be
    /// another message standard/protocol, network, database, shared table, etc. UCI leaves it to
    /// implementers to choose and coordinate use of the foreign "system".
    #[serde(rename = "SystemName")]
    pub system_name: VisibleString256Type,
}

/// The ID type for UCI IDs that correspond to EW_Configurations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EwConfigurationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// Defines the Earth Orientation Parameters ID.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EarthOrientationParametersIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Effects.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EffectIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// The ID type for UCI IDs that correspond to EffectPlanCommand messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EffectPlanCommandIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to EffectPlan messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EffectPlanIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// The ID type for UCI IDs that correspond to EffectPlanValidation messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EffectPlanValidationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to EOB emitter mode instances.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EmitterModeIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Emitter Records.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EmitterRecordIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to entities.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EntityIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type that correspond to a CSO for an orbiting entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EntityOrbitalCsoIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to an orbital element set (TLE).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EntityOrbitalElementSetIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to an ephemeris.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EntityOrbitalEphemerisIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to an orbital vector covariance matrix (VCM) and
/// associated parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EntityOrbitalVcmIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionPlanSetIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that corresponds to an external data record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalRequestedDataIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// Encoding types for CVEnumISMCATFGIOpen Version 2 controlled vocabulary enumerations. Derived
/// from the CVEnumISMCATFGIOpen.xml CVE.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum FgiSourceOpenChoiceType {
    /// CVEnumISMCATFGIOpen Values
    ForeignGovernmentIdentifier {
        /// CVEnumISMCATFGIOpen Values
        #[serde(rename = "ForeignGovernmentIdentifier")]
        foreign_government_identifier: FgiSourceOpenEnum,
    },
    /// North Atlantic Treaty Organization Special Words
    NatoSpecialWord {
        /// North Atlantic Treaty Organization Special Words
        #[serde(rename = "NATO_SpecialWord")]
        nato_special_word: NatoSpecialWordsType,
    },
}

/// Encoding types for CVEnumISMCATFGIProtected Version 2.1 controlled vocabulary enumerations.
/// Derived from the CVEnumISMCATFGIProtected.xml CVE.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum FgiSourceProtectedChoiceType {
    /// CVEnumISMCATFGIProtected Values
    ForeignGovernmentIdentifier {
        /// CVEnumISMCATFGIProtected Values
        #[serde(rename = "ForeignGovernmentIdentifier")]
        foreign_government_identifier: FgiSourceProtectedEnum,
    },
    /// North Atlantic Treaty Organization Special Words
    NatoSpecialWord {
        /// North Atlantic Treaty Organization Special Words
        #[serde(rename = "NATO_SpecialWord")]
        nato_special_word: NatoSpecialWordsType,
    },
}

/// The ID type for UCI IDs that correspond to FacilityCharacteristics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FacilityCharacteristicsIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Facility Records.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FacilityRecordIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// A fault id should be unique for every fault. DescriptiveLabel should have as much detailed
/// information as possible about the fault. The name should be something short that fits nicely in
/// a display where as description can be long and help the user really understand the fault. For
/// example, DescriptiveLabel - "Temp Over Heat in Oil pump engine 1 inlet", Name - "Temp Over Heat
/// Eng 1".
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FaultIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// A short name used to display to an operator if the fault code is not meaningful to a human and
    /// in the case where the fault description is too long.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<VisibleString64Type>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FileLocationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FileMetadataIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for a gateway message filter record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FilterIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to flight rules.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FlightRuleIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// Contains the ForeignKeyMap Data Record List (DRL).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ForeignKeyMapIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// The ID type for UCI IDs that correspond to ForeignKeyPairs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ForeignKeyPairIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ForeignKeyType {
    /// This element contains the value of the ForeignKey.
    #[serde(rename = "Key")]
    pub key: VisibleString256Type,
    /// This element refers to the "system" where the ForeignKey is managed. That system could be
    /// another message standard/protocol, network, database, shared table, etc. UCI leaves it to
    /// implementers to choose and coordinate use of the foreign "system".
    #[serde(rename = "SystemName")]
    pub system_name: VisibleString256Type,
}

/// The ID type that refers to an aggregation of Subsystems.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FunctionIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Gimbal Axis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GimbalAxisIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// This type is for the Header used by all messages in the UCI interface.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeaderType {
    /// This element represents the SystemID of the System that sent this message.
    #[serde(rename = "SystemID")]
    pub system_id: SystemIdType,
    /// This element represents the time and date at which the message was created and ready to be sent.
    #[serde(rename = "Timestamp")]
    pub timestamp: DateTimeType,
    /// Indicates the schema version used to generate the message. UCI v2.0 would be indicated as
    /// "002.0", UCI v1.9b would be indicated as "001.9b".
    #[serde(rename = "SchemaVersion")]
    pub schema_version: UciSchemaVersionStringType,
    /// This enumerated element indicates the "message mode" of the message. See enumeration values for
    /// details.
    #[serde(rename = "Mode")]
    pub mode: MessageModeEnum,
    /// This element represents the ServiceID of the Service that sent this message.
    #[serde(rename = "ServiceID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<ServiceIdType>,
    /// Defines the Mission that this message was generated for and applies to. Required if the message
    /// is Mission unique. See the Mission message which defines a Mission.
    #[serde(rename = "MissionID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mission_id: Option<MissionIdType>,
}

/// The common base type for all UCI IDs. All extension types should end in "ID_Type".
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to ObservationMeasurementReport messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IffMeasurementAndDataMessageIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to ObservationMeasurementReport messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IffMeasurementAndDataReportMessageIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to ObservationMeasurementReport messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IffMeasurementIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Jam Strobe Report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JamStrobeReportIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to LARs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LarIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to LandCharacteristics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LandCharacteristicsIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Land Records.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LandRecordIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to a launch object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LaunchObjectIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to LaunchObservation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LaunchObservationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MdfIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MdfSubCategoryFileIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MdfSubCategoryIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// ID observed with a measurement group.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MeasurementGroupIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// ID observed with a measurement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MeasurementIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// Identifies a Message Configuration ID.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MessageConfigurationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// Identifies a Message Transmission List Entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MessageTransmissionFilterRecordIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// Identifies a Message Transmission List.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MessageTransmissionListIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MessageType {
    /// The security markings applicable to this message.
    #[serde(rename = "SecurityInformation")]
    pub security_information: SecurityInformationType,
    /// Common descriptive and control information that applies to any message.
    #[serde(rename = "MessageHeader")]
    pub message_header: HeaderType,
}

/// The ID type for UCI IDs that correspond to Metadata messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MetadataIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to MissileCharacteristics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MissileCharacteristicsIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Missile Records.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MissileRecordIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MissionContingencyAlertIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MissionEnvironmentOverrideIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to a mission.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MissionIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// The ID type for UCI IDs that correspond to mission activation commands.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MissionPlanActivationCommandIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to MissionPlanCommand messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MissionPlanCommandIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to MissionPlan messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MissionPlanIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// The ID type for UCI IDs that correspond to MissionPlanMetricsReport messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MissionPlanMetricsReportIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// The ID type for UCI IDs that correspond to MissionPlanValidation messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MissionPlanValidationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to MissionPlanningOrchestration messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MissionPlanningOrchestrationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to the MissionValidation messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MissionValidationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// Indicates the unique ID of the emitter data corresponding to this message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MultistaticEmitterDataIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// Indicates the unique ID of the specified emitter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MultistaticEmitterIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MvAnomalyEventIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkLinkIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to a notification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NotificationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to OB correlation records.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ObCorrelationRecordIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to the ID of the observation measurement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ObservationMeasurementIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to ObservationMeasurementReport messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ObservationMeasurementReportIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to ObservationReports.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ObservationReportIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to OpLines.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpLineIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to op lines.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpPointIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to op routing messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpRoutingIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to an OpVolume.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpVolumeIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to op zones.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpZoneIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Operator Actions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperatorActionIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type that refers to a specific operator annotation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperatorAnnotationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type that refers to an actual operator.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperatorIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Operator Locations of Interest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperatorLocationOfInterestIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Operator Notifications.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperatorNotificationAckIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Operator Notifications.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperatorNotificationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to this OperatorNotificationOverride.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperatorNotificationOverrideIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Operator Roles.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperatorRoleIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for Operator stations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperatorStationIdentifierType {
    /// This element contains the value of the ForeignKey.
    #[serde(rename = "Key")]
    pub key: VisibleString256Type,
    /// This element refers to the "system" where the ForeignKey is managed. That system could be
    /// another message standard/protocol, network, database, shared table, etc. UCI leaves it to
    /// implementers to choose and coordinate use of the foreign "system".
    #[serde(rename = "SystemName")]
    pub system_name: VisibleString256Type,
}

/// The ID type for UCI IDs that correspond to OrbitActivityPlan commands.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrbitActivityPlanCommandIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to OrbitActivityPlans.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrbitActivityPlanIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// The ID type for UCI IDs that correspond to OrbitActivityPlanValidation messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrbitActivityPlanValidationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Orbit Conjunction Assessments.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrbitConjunctionAssessmentIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The unique ID that identifies a specific orbit kinematics sequence.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrbitKinematicsSequenceIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// The ID type for UCI IDs that corresponds to an Orbit Plan Maneuver Segment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrbitManeuverSegmentIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Orbit Metrics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrbitMetricsIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// The ID type for UCI IDs that correspond to Orbit plan commands.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrbitPlanCommandIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Orbit plans.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrbitPlanIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// The ID type for UCI IDs that correspond to OrbitPlanValidation messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrbitPlanValidationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// Indicates the unique ID of a previously identified orbital conjunction that is the basis for
/// this debris estimate request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrbitalConjunctionIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// Indicates the unique ID of an orbital debris cloud.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrbitalDebrisCloudIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to OrbitalDebris estimates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrbitalDebrisEstimateIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// Indicates the unique ID of a particular piece of orbital debris which is associated with its
/// unique characteristics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrbitalDebrisIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// This type is the standard UCI ID type that correspond to an orbital maneuver.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrbitalManeuverIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to OrderOfBattle.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderOfBattleIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to OrdersMetadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersMetadataIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// Encoding types for CVEnumISMCATOwnerProducer Version 2 controlled vocabulary enumerations.
/// Derived from the CVEnumISMCATOwnerProducer.xml CVE.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum OwnerProducerChoiceType {
    /// CVEnumISMCATOwnerProducer Values
    GovernmentIdentifier {
        /// CVEnumISMCATOwnerProducer Values
        #[serde(rename = "GovernmentIdentifier")]
        government_identifier: OwnerProducerEnum,
    },
    /// North Atlantic Treaty Organization Special Words
    NatoSpecialWord {
        /// North Atlantic Treaty Organization Special Words
        #[serde(rename = "NATO_SpecialWord")]
        nato_special_word: NatoSpecialWordsType,
    },
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PoApertureIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PoFpaChannelIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PoFpaIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PoFilterIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PoLensIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PoProcessingStageIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PoProductGeneratorIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PoProductGeneratorOutputIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PoWindowIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Packages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that corresponds to PackageReadiness.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageReadinessIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to configuration parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ParameterIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to paths in a mission plan's routes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PathIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// This type is the standard UCI ID type that corresponds to the Plan Scores.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlanScoresIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Planned Activity Events.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlannedActivityEventIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to planned Activities.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlannedActivityIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to PlanningFunctions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlanningFunctionIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to planning processes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlanningProcessIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Platform Records.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlatformRecordIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to platform *Report messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlatformReportIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// The ID type for UCI IDs that correspond to a List of Target Of Opportunity Priority List.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrioritizationListIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to ProductDownloadPlan.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductDownloadPlanIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to ProductDownloadReport.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductDownloadReportIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to product location.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductLocationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to product metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductMetadataIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to ProductOrFileClassificationPlan.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductOrFileClassificationPlanIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to a ProductOrFileClassificationReport.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductOrFileClassificationReportIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to ProductOrFileDisseminationConfiguration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductOrFileDisseminationConfigurationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Product Dissemination Destination IDs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductOrFileDisseminationDestinationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to ProductOrFileDisseminationPlan.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductOrFileDisseminationPlanIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to ProductOrFileDisseminationReport.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductOrFileDisseminationReportIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to ProductProcessingPlan.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductProcessingPlanIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to ProductProcessingReport.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductProcessingReportIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// This element defines the ID for propagator settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PropagatorSettingsIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to signals intelligence pulse data messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PulseDataIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to RDMA Initialize messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RdmaInitializeIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to unique RDMA initialization setup.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RdmaInitializeSetupIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// Foreign key of the RFD component inside the RFDC subsystem.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RfdComponentIdentifierType {
    /// This element contains the value of the ForeignKey.
    #[serde(rename = "Key")]
    pub key: VisibleString256Type,
    /// This element refers to the "system" where the ForeignKey is managed. That system could be
    /// another message standard/protocol, network, database, shared table, etc. UCI leaves it to
    /// implementers to choose and coordinate use of the foreign "system".
    #[serde(rename = "SystemName")]
    pub system_name: VisibleString256Type,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RfProfileIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RfReportIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to RF_ResourceAllocation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RfResourceAllocationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to the ID of the radar altimeter measurement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RadarAltimeterReportIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Order of Battle Record (OBR) objects.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to reference frames.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReferenceFrameIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to DisplayRelationships.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RelationshipDesignationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// Encoding types for CVEnumISMCATRelTo Version 2 controlled vocabulary enumerations. Derived from
/// the CVEnumISMCATRelTo.xml CVE.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum ReleasableToChoiceType {
    /// CVEnumISMCATRelTo Values
    GovernmentIdentifier {
        /// CVEnumISMCATRelTo Values
        #[serde(rename = "GovernmentIdentifier")]
        government_identifier: ReleasableToEnum,
    },
    /// North Atlantic Treaty Organization Special Words
    NatoSpecialWord {
        /// North Atlantic Treaty Organization Special Words
        #[serde(rename = "NATO_SpecialWord")]
        nato_special_word: NatoSpecialWordsType,
    },
}

/// The ID type for UCI IDs that correspond to command messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Requirement allocation windows.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequirementAllocationWindowIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Requirement Metrics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequirementMetricsIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// The ID type for UCI IDs that correspond to RequirementOptions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequirementOptionsIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// The ID type for UCI IDs that correspond to RequirementSets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequirementSetIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to a RequirementsTemplate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequirementsTemplateIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Response messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResponseIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
    /// Indicates a specific Option within the Response.
    #[serde(rename = "OptionIndex")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub option_index: Vec<u32>,
}

/// The ID type for UCI IDs that correspond to ResponsePlanCommand messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResponsePlanCommandIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to ResponsePlan messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResponsePlanIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// The ID type for UCI IDs that correspond to ResponsePlanValidation messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResponsePlanValidationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to RouteActivityPlanCommand messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RouteActivityPlanCommandIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to RouteActivityPlan messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RouteActivityPlanIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// The ID type for UCI IDs that correspond to RouteActivityPlanValidation messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RouteActivityPlanValidationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Route Metrics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RouteMetricsIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// The ID type for UCI IDs that correspond to RoutePlanCommand messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RoutePlanCommandIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to RoutePlan messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RoutePlanIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// The ID type for UCI IDs that correspond to RoutePlanValidation messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RoutePlanValidationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to runways.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunwayIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to a Satellite's Physical Configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SatellitePhysicalConfigurationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Scoring Processes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScoringProcessIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to SeaSubsurfaceCharacteristics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SeaSubsurfaceCharacteristicsIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to SeaSubsurface Records.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SeaSubsurfaceRecordIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to SeaSurfaceCharacteristics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SeaSurfaceCharacteristicsIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to SeaSurface Records.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SeaSurfaceRecordIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// This type indicates a variety of security information for a data asset including how a data
/// asset shall be stored, protected, and destroyed. This type is based on IC ISM
/// SecurityAttributesGroup IC-ISM.xsd.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecurityInformationType {
    /// A single indicator of the highest level of classification applicable to an information resource
    /// or portion within the domain of classified national security information.
    #[serde(rename = "Classification")]
    pub classification: ClassificationEnum,
    /// Identifies the national government or international organization that have purview over the
    /// classification marking of an information resource or portion therein. Within protected internal
    /// organizational spaces this element may include up to 1000 indicators identifying information
    /// which qualifies as foreign government information for which the source(s) of the information
    /// must be concealed. Measures must be taken prior to dissemination of the information to conceal
    /// the source(s) of the foreign government information. Specifically, under these specific
    /// circumstances, when data are moved to the shared spaces, the non-disclosable owner(s) and/or
    /// producer(s) listed in this data element's value should be removed and replaced with "FGI".
    #[serde(rename = "OwnerProducer")]
    pub owner_producer: Vec<OwnerProducerChoiceType>,
    /// When true, is used to signify that multiple values in the OwnerProducer element are JOINT owners
    /// of the data.
    #[serde(rename = "Joint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joint: Option<bool>,
    /// Identifies sensitive compartmented information control system(s). List size for this element is
    /// based on "Select All That Apply" condition.
    #[serde(rename = "SCI_Controls")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sci_controls: Vec<SciControlsEnum>,
    /// Identifies the defense or intelligence programs for which special access is required.
    #[serde(rename = "SAR_Identifier")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sar_identifier: Vec<WhitespaceVisibleString4096Type>,
    /// Identifies DoE markings. List size for this element is based on "Select All That Apply"
    /// condition.
    #[serde(rename = "AtomicEnergyMarkings")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub atomic_energy_markings: Vec<AtomicEnergyMarkingsEnum>,
    /// Identifies the expansion or limitation on the distribution of information. List size for this
    /// element is based on "Select All That Apply" condition.
    #[serde(rename = "DisseminationControls")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dissemination_controls: Vec<DisseminationControlsEnum>,
    /// Identifies the country/countries and/or international organization(s) to which classified
    /// information may be displayed but NOT released based on the determination of an originator in
    /// accordance with established foreign disclosure procedures. This element is used in conjunction
    /// with the DisplayOnly Dissemination Controls value.
    #[serde(rename = "DisplayOnlyTo")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub display_only_to: Vec<ReleasableToChoiceType>,
    /// Identifies information which qualifies as foreign government information for which the source(s)
    /// of the information is not concealed. The attribute can indicate that the source of information
    /// of foreign origin is UNKNOWN.
    #[serde(rename = "FGI_SourceOpen")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fgi_source_open: Vec<FgiSourceOpenChoiceType>,
    /// Identifies information which qualifies as foreign government information for which the source(s)
    /// of the information must be concealed. The attribute can indicate that the source of information
    /// of foreign origin is UNKNOWN.
    #[serde(rename = "FGI_SourceProtected")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fgi_source_protected: Vec<FgiSourceProtectedChoiceType>,
    /// Identifies the country or countries and/or international organization(s) to which classified
    /// information may be released based on the determination of an originator in accordance with
    /// established foreign disclosure procedures. This element is used in conjunction with the
    /// Dissemination Controls element.
    #[serde(rename = "ReleasableTo")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub releasable_to: Vec<ReleasableToChoiceType>,
    /// Identifies the expansion or limitation on the distribution of an information resource or portion
    /// within the domain of information originating from non-intelligence components. List size for
    /// this element is based on "Select All That Apply" condition.
    #[serde(rename = "NonIC_Markings")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub non_ic_markings: Vec<NonIcMarkingsEnum>,
    /// Identifies, by name or personal identifier, and position title of the original classification
    /// authority for a resource.
    #[serde(rename = "ClassifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classified_by: Option<WhitespaceVisibleString1024Type>,
    /// A description of the reasons that the classification of this element is more restrictive than a
    /// simple roll-up of the sub elements would result in. This acts as an indicator to rule engines
    /// that there is not accidental over classification going on and to users that special care beyond
    /// what the portion marks reveal must be taken when using this data. Use of this mark does not
    /// replace the need for the compilation reason being defined in the prose in accordance with ISOO
    /// Directive 1. For example this would document why 3 Unclassified bullet items form a Secret List.
    /// Without this reason being noted the above described document would be considered to be
    /// miss-marked and overclassified.
    #[serde(rename = "CompilationReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compilation_reason: Option<WhitespaceVisibleString1024Type>,
    /// Identifies, by name or personal identifier, of the derivative classification authority.
    #[serde(rename = "DerivativelyClassifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub derivatively_classified_by: Option<WhitespaceVisibleString1024Type>,
    /// This element is used primarily at the resource level. One or more reason indicators or
    /// explanatory text describing the basis for an original classification decision. It is manifested
    /// only in the 'Reason' line of a resource's classification authority block.
    #[serde(rename = "ClassificationReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_reason: Option<WhitespaceVisibleString4096Type>,
    /// One or more indicators of the expansion or limitation on the distribution of an information
    /// resource or portion within the domain of information originating from non-US components. List
    /// size for this element is based on "Select All That Apply" condition.
    #[serde(rename = "NonUS_Controls")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub non_us_controls: Vec<NonUsControlsEnum>,
    /// A citation of the authoritative source or reference to multiple sources of the classification
    /// markings used in a classified resource. It is manifested only in the 'Derived From' line of a
    /// document's classification authority block. ISOO's guidance is: Source of derivative
    /// classification. (1) The derivative classifier shall concisely identify the source document or
    /// the classification guide on the "Derived From" line, including the agency and, where available,
    /// the office of origin, and the date of the source or guide. An example might appear as: Derived
    /// From: Memo, "Funding Problems," October 20, 2008, Office of Administration, Department of Good
    /// Works or Derived From: CG No. 1, Department of Good Works, dated October 20, 2008 (i) When a
    /// document is classified derivatively on the basis of more than one source document or
    /// classification guide, the "Derived From" line shall appear as: Derived From: Multiple Sources
    /// (ii) The derivative classifier shall include a listing of the source materials on, or attached
    /// to, each derivatively classified document.
    #[serde(rename = "DerivedFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub derived_from: Option<WhitespaceVisibleString1024Type>,
    /// A specific year, month, and day upon which the information shall be automatically declassified
    /// if not properly exempted from automatic declassification.
    #[serde(rename = "DeclassDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declass_date: Option<String>,
    /// A description of an event upon which the information shall be automatically declassified if not
    /// properly exempted from automatic declassification.
    #[serde(rename = "DeclassEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declass_event: Option<WhitespaceVisibleString1024Type>,
    /// A single indicator describing an exemption to the nominal 25-year point for automatic
    /// declassification. This element is used in conjunction with the Declassification Date or
    /// Declassification Event. ISOO has stated it should be a SINGLE value giving the longest
    /// protection. List size for this element is based on "Select All That Apply" condition.
    #[serde(rename = "DeclassException")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub declass_exception: Vec<DeclassExceptionEnum>,
    /// When true, indicates the ISM markings specified are estimated (e.g., system high).
    #[serde(rename = "HasApproximateMarkings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_approximate_markings: Option<bool>,
    /// Applicable NATO highwater markings for a document or portion. List size for this element is
    /// based on "Select All That Apply" condition.
    #[serde(rename = "HighWaterNATO")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub high_water_nato: Vec<HighWaterNatoEnum>,
    /// Applicable CUI Basic markings for a document or portion. List size for this element is based on
    /// "Select All That Apply" condition.
    #[serde(rename = "CUI_Basic")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cui_basic: Vec<CuiBasicEnum>,
    /// Applicable CUI Specified markings for a document or portion. List size for this element is based
    /// on "Select All That Apply" condition.
    #[serde(rename = "CUI_Specified")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cui_specified: Vec<CuiSpecifiedEnum>,
    /// The specific date when a CUI resource is subject to automatic CUI decontrol procedures.
    #[serde(rename = "CUI_DecontrolDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cui_decontrol_date: Option<String>,
    /// A description of an event upon which CUI information shall be subject to automatic decontrol
    /// procedures.
    #[serde(rename = "CUI_DecontrolEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cui_decontrol_event: Option<WhitespaceVisibleString1024Type>,
    /// The identity, by name or personal identifier and position title, of the CUI controlling
    /// authority for a document containing CUI information.
    #[serde(rename = "CUI_ControlledBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cui_controlled_by: Option<WhitespaceVisibleString1024Type>,
    /// Office in an agency or department that is responsible for labeling and controlling information
    /// as CUI.
    #[serde(rename = "CUI_ControlledByOffice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cui_controlled_by_office: Option<WhitespaceVisibleString1024Type>,
    /// Point of Contact for labeling and controlling information as CUI.
    #[serde(rename = "CUI_POC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cui_poc: Option<WhitespaceVisibleString1024Type>,
    /// Tokens that contain markings used to support administrative and legal processes for handling and
    /// protecting documents. When they appear in a document, these tokens form a second line that is
    /// placed below the banner. List size for this element is based on "Select All That Apply"
    /// condition.
    #[serde(rename = "SecondBannerLine")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub second_banner_line: Vec<SecondBannerLineEnum>,
    /// Handle VIA Channels that may appear in the second banner line.
    #[serde(rename = "HandleViaChannels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle_via_channels: Option<WhitespaceVisibleString4096Type>,
}

/// The ID type for UCI IDs that correspond to segments in mission plans routes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SegmentIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// The ID type for UCI IDs that correspond to order of battle sensors.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SensorIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to service error reports.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceErrorReportIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to services that communicate using the UCI protocol.
/// Services have a digital network location which is represented by this ID.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// This field should contain the version number (i.e. 1.2.2) of this service. The Descriptive Label
    /// contains the human readable service name.
    #[serde(rename = "ServiceVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_version: Option<VisibleString256Type>,
}

/// See annotations in child elements and associated global element (message) for further details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceLifecycleCommandMdt {
    /// Indicates the unique ID of the Command corresponding to this message. Used to reference the
    /// command when providing status.
    #[serde(rename = "CommandID")]
    pub command_id: CommandIdType,
    /// The state of the command message data. Indicates if a command message is a NEW command, an
    /// UPDATE to an existing command, or a CANCEL of an existing command.
    #[serde(rename = "CommandState")]
    pub command_state: CommandStateEnum,
    /// Mission that the service is to be associated.
    #[serde(rename = "MissionID")]
    pub mission_id: MissionIdType,
    /// If specified, represents the ID of the service that should effect this request.
    #[serde(rename = "EffectingServiceID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effecting_service_id: Option<ServiceIdType>,
    /// Represents the ID of the services that should be affected by this request.
    #[serde(rename = "AffectedServiceID")]
    pub affected_service_id: Vec<ServiceIdType>,
    /// The type of command issued: start, stop, restart.
    #[serde(rename = "LifecycleState")]
    pub lifecycle_state: ServiceLifecycleStateEnum,
}

/// See the associated global element (message) annotation for further details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceLifecycleCommandMt {
    /// The security markings applicable to this message.
    #[serde(rename = "SecurityInformation")]
    pub security_information: SecurityInformationType,
    /// Common descriptive and control information that applies to any message.
    #[serde(rename = "MessageHeader")]
    pub message_header: HeaderType,
    /// See annotations in child elements for details.
    #[serde(rename = "MessageData")]
    pub message_data: ServiceLifecycleCommandMdt,
}

/// See annotations in child elements and associated global element (message) for further details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceLifecycleCommandStatusMdt {
    /// Indicates the unique ID of the command corresponding to this status message.
    #[serde(rename = "CommandID")]
    pub command_id: CommandIdType,
    /// Indicates the state of the command. The command state machine assumes that the Command and
    /// CommandStatus messaging interaction is only used to communicate the desired Command to the
    /// Subsystem or Service. The state machine and messaging terminates as soon as the host accepts or
    /// rejects the Command. See enumeration annotations for further details.
    #[serde(rename = "CommandProcessingState")]
    pub command_processing_state: CommandProcessingStateEnum,
    /// Indicates details regarding the reason for and/or a contributing factor to the sibling
    /// CommandProcessingState element. This element is only expected when the Command is in a
    /// potentially undesirable state such as rejected, canceled or accepted with less than optimum
    /// results expected.
    #[serde(rename = "CommandProcessingStateReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_processing_state_reason: Option<CannotComplyTypeValue>,
    /// The status of the command that was executed.
    #[serde(rename = "LifecycleCommandStatus")]
    pub lifecycle_command_status: ServiceLifecycleCommandStatusEnum,
    /// Represents the ID of the service that processed this command.
    #[serde(rename = "ServiceID")]
    pub service_id: ServiceIdType,
}

/// See the associated global element (message) annotation for further details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceLifecycleCommandStatusMt {
    /// The security markings applicable to this message.
    #[serde(rename = "SecurityInformation")]
    pub security_information: SecurityInformationType,
    /// Common descriptive and control information that applies to any message.
    #[serde(rename = "MessageHeader")]
    pub message_header: HeaderType,
    /// See annotations in child elements for details.
    #[serde(rename = "MessageData")]
    pub message_data: ServiceLifecycleCommandStatusMdt,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceSettingType {
    /// Indicates the supported Service setting. See enumeration annotations for further details.
    #[serde(rename = "ServiceSetting")]
    pub service_setting: ServiceSettingEnum,
    /// Indicates the supported parameter/level at which the sibling ServiceSetting is being used.
    #[serde(rename = "ServiceSettingParameter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_setting_parameter: Option<ServiceSettingsParameterType>,
    /// Valid when ServiceSetting is AUDIT_LOGGING; 1..n may be valid for Audit logging. List size for
    /// this element is based on "Select All That Apply" condition.
    #[serde(rename = "EventSeverityClass")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event_severity_class: Vec<EventSeverityEnum>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceSettingsParameterType {
    /// Indicates the Service settings parameter's identifying number/index.
    #[serde(rename = "ParameterNumber")]
    pub parameter_number: u32,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString1024Type>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceStateWindowType {
    /// This element represents the state of the Service for the sibling time window. See enumeration
    /// annotation for further details.
    #[serde(rename = "ServiceState")]
    pub service_state: ServiceStateEnum,
    /// Indicates the time window when the sibling state is predicted/expected.
    #[serde(rename = "Window")]
    pub window: DateTimeRangeType,
    /// Indicates details regarding the reason for and/or a contributing factor to the sibling predicted
    /// state.
    #[serde(rename = "StateReason")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub state_reason: Vec<CannotComplyTypeValue>,
}

/// See annotations in child elements and associated global element (message) for further details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceStatusMdt {
    /// The ID of the service.
    #[serde(rename = "ServiceID")]
    pub service_id: ServiceIdType,
    /// Amount of time service has been running.
    #[serde(rename = "TimeUp")]
    pub time_up: DurationType,
    /// Indicates the current state of the Service. The state machine and allowable transitions for
    /// Service states is described in supporting documentation. A given Service might not support all
    /// states. Service state may be controlled via ServiceLifecycleCommand. See enumeration annotations
    /// for further details.
    #[serde(rename = "ServiceState")]
    pub service_state: ServiceStateEnum,
    /// Indicates details regarding the reason for and/or a contributing factor to the sibling
    /// ServiceState.
    #[serde(rename = "ServiceStateReason")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_state_reason: Vec<CannotComplyTypeValue>,
    /// This element represents a predicted, scheduled, or otherwise expected state of the service for a
    /// future time window.
    #[serde(rename = "PredictedServiceState")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub predicted_service_state: Vec<ServiceStateWindowType>,
    /// List of current enabled settings for the service.
    #[serde(rename = "EnabledSettings")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub enabled_settings: Vec<ServiceSettingType>,
    /// Full list of available settings that the service may support if enabled.
    #[serde(rename = "SupportedSettings")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supported_settings: Vec<ServiceSettingType>,
}

/// See the associated global element (message) annotation for further details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceStatusMt {
    /// The security markings applicable to this message.
    #[serde(rename = "SecurityInformation")]
    pub security_information: SecurityInformationType,
    /// Common descriptive and control information that applies to any message.
    #[serde(rename = "MessageHeader")]
    pub message_header: HeaderType,
    /// See annotations in child elements for details.
    #[serde(rename = "MessageData")]
    pub message_data: ServiceStatusMdt,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignalIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignalReportIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to SpaceCharacteristics messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpaceCharacteristicsIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Space Records.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpaceRecordIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// This element defines the ID for Space Weather.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpaceWeatherIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to SpectralDensityReport.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpectralDensityReportIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to StoreLoadoutConfigurations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StoreLoadoutConfigurationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to StoreManagementReport.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StoreManagementReportIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to resource capabilities.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SubCapabilityIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to SubsystemBIT_Configurations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SubsystemBitConfigurationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to SubsystemCalibrationConfigurations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SubsystemCalibrationConfigurationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to SubsystemConfigurations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SubsystemConfigurationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type that refers to an aggregation of Capabilities.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SubsystemIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// Indicates the unique ID of a SubsystemMaintenanceConfiguration message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SubsystemMaintenanceConfigurationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// Indicates the unique ID of a maintenance test.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SubsystemMaintenanceIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type that refers to a commandable and/or autonomous supporting element of a Capability.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SupportCapabilityIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to SupportPlan messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SupportPlanIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// Indicates the unique ID of the SupportCapability corresponding to this message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SupportedResolutionIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SurvivabilityRiskLevelIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type that refers to an aggregation of Subsystems.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SystemIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to an orbital element set (TLE) for a System.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SystemOrbitalElementSetIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to an ephemeris.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SystemOrbitalEphemerisIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to an orbital vector covariance matrix (VCM) and
/// associated parameters for a System.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SystemOrbitalVcmIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI ID that correspond to a SystemReadiness.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SystemReadinessIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to TagAssociations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TagAssociationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Tasks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// The ID type for UCI IDs that correspond to TaskPlanCommand messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskPlanCommandIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to TaskPlan messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskPlanIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// The ID type for UCI IDs that correspond to TaskPlanValidation messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskPlanValidationIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to resource capabilities.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TrackOptionsIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to TurretReports.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TurretReportIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// Unique Material ID (UMID) Audio.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UmidAudioIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// Unique Material ID (UMID) Data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UmidDataIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// Unique Material ID (UMID) System.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UmidSystemIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// Unique Material ID (UMID) Video.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UmidVideoIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// See annotations in child elements and messages/elements that use this type for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnallocatedReasonType {
    /// Indicates the reason why an action (Task, \[Capability\]Command,
    /// \[SupportingCapability\]Command, \[Capability\]Activity, etc.) was rejected, interrupted,
    /// unallocated, failed and/or generally can't be completed satisfactorily. See enumeration
    /// annotations for further details.
    #[serde(rename = "Reason")]
    pub reason: CannotComplyEnum,
    /// Indicates a human-readable text description of the reason for the "can't comply".
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<VisibleString1024Type>,
    /// Indicates an ID corresponding to another object that is associated with or the cause of the
    /// "can't comply".
    #[serde(rename = "AssociatedID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_id: Option<IdTypeValue>,
    /// Indicates the unique ID of a System that a Requirement couldn't be allocated to.
    #[serde(rename = "SystemID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_id: Option<SystemIdType>,
}

/// Indicates the Unit UUID.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnitIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that correspond to Unit Records.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnitRecordIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// The ID type for UCI IDs that use versioning.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VersionedIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
    /// Indicates an incremental version of an item in which only minor changes have been made. An
    /// example of a change that would require a new UUID and not just a new version would be for
    /// changes that affect task allocations or changes that require re-approval.
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
}

/// The ID type for UCI IDs that correspond to weather datasets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WeatherDatasetIdType {
    /// Universally Unique Identifier (UUID). A UUID is a 128-bit number (32 hexadecimal digits, 16
    /// bytes) that is conformant to a version and variant of IETF RFC 4122.
    #[serde(rename = "UUID")]
    pub uuid: UniversallyUniqueIdentifierType,
    /// Human readable text label with content meaningful to human operators.
    #[serde(rename = "DescriptiveLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<VisibleString256Type>,
}

/// A value in a `CannotComplyType` element position, including legal derived types.
#[derive(Debug, Clone, PartialEq)]
pub enum CannotComplyTypeValue {
    /// Carry `CannotComplyType`.
    CannotComplyType(CannotComplyType),
    /// Carry `UnallocatedReasonType`.
    UnallocatedReasonType(UnallocatedReasonType),
}
impl Serialize for CannotComplyTypeValue {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let (mut value, tag) = match self {
            Self::CannotComplyType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "CannotComplyType",
            ),
            Self::UnallocatedReasonType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "UnallocatedReasonType",
            ),
        };
        if tag != "CannotComplyType" {
            value
                .as_object_mut()
                .ok_or_else(|| serde::ser::Error::custom("expected object"))?
                .insert("$type".into(), tag.into());
        }
        value.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for CannotComplyTypeValue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let mut value = serde_json::Value::deserialize(deserializer)?;
        let tag = value
            .as_object_mut()
            .ok_or_else(|| serde::de::Error::custom("expected object"))?
            .remove("$type");
        let tag = match &tag {
            None => "CannotComplyType",
            Some(serde_json::Value::String(s)) => s.as_str(),
            _ => return Err(serde::de::Error::custom("$type must be a string")),
        };
        match tag {
            "CannotComplyType" => serde_json::from_value(value)
                .map(Self::CannotComplyType)
                .map_err(serde::de::Error::custom),
            "UnallocatedReasonType" => serde_json::from_value(value)
                .map(Self::UnallocatedReasonType)
                .map_err(serde::de::Error::custom),
            _ => Err(serde::de::Error::custom(format!(
                "illegal derived type {tag}"
            ))),
        }
    }
}

/// A value in a `ForeignKeyType` element position, including legal derived types.
#[derive(Debug, Clone, PartialEq)]
pub enum ForeignKeyTypeValue {
    /// Carry `ForeignKeyType`.
    ForeignKeyType(ForeignKeyType),
    /// Carry `CallSignType`.
    CallSignType(CallSignType),
    /// Carry `CommAntennaModeType`.
    CommAntennaModeType(CommAntennaModeType),
    /// Carry `EA_TechniqueIdentifierType`.
    EaTechniqueIdentifierType(EaTechniqueIdentifierType),
    /// Carry `EOB_EquipmentType`.
    EobEquipmentType(EobEquipmentType),
    /// Carry `EOB_FacilityPIN_Type`.
    EobFacilityPinType(EobFacilityPinType),
    /// Carry `OperatorStationIdentifierType`.
    OperatorStationIdentifierType(OperatorStationIdentifierType),
    /// Carry `RFD_ComponentIdentifierType`.
    RfdComponentIdentifierType(RfdComponentIdentifierType),
}
impl Serialize for ForeignKeyTypeValue {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let (mut value, tag) = match self {
            Self::ForeignKeyType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ForeignKeyType",
            ),
            Self::CallSignType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "CallSignType",
            ),
            Self::CommAntennaModeType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "CommAntennaModeType",
            ),
            Self::EaTechniqueIdentifierType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "EA_TechniqueIdentifierType",
            ),
            Self::EobEquipmentType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "EOB_EquipmentType",
            ),
            Self::EobFacilityPinType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "EOB_FacilityPIN_Type",
            ),
            Self::OperatorStationIdentifierType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "OperatorStationIdentifierType",
            ),
            Self::RfdComponentIdentifierType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "RFD_ComponentIdentifierType",
            ),
        };
        if tag != "ForeignKeyType" {
            value
                .as_object_mut()
                .ok_or_else(|| serde::ser::Error::custom("expected object"))?
                .insert("$type".into(), tag.into());
        }
        value.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for ForeignKeyTypeValue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let mut value = serde_json::Value::deserialize(deserializer)?;
        let tag = value
            .as_object_mut()
            .ok_or_else(|| serde::de::Error::custom("expected object"))?
            .remove("$type");
        let tag = match &tag {
            None => "ForeignKeyType",
            Some(serde_json::Value::String(s)) => s.as_str(),
            _ => return Err(serde::de::Error::custom("$type must be a string")),
        };
        match tag {
            "ForeignKeyType" => serde_json::from_value(value)
                .map(Self::ForeignKeyType)
                .map_err(serde::de::Error::custom),
            "CallSignType" => serde_json::from_value(value)
                .map(Self::CallSignType)
                .map_err(serde::de::Error::custom),
            "CommAntennaModeType" => serde_json::from_value(value)
                .map(Self::CommAntennaModeType)
                .map_err(serde::de::Error::custom),
            "EA_TechniqueIdentifierType" => serde_json::from_value(value)
                .map(Self::EaTechniqueIdentifierType)
                .map_err(serde::de::Error::custom),
            "EOB_EquipmentType" => serde_json::from_value(value)
                .map(Self::EobEquipmentType)
                .map_err(serde::de::Error::custom),
            "EOB_FacilityPIN_Type" => serde_json::from_value(value)
                .map(Self::EobFacilityPinType)
                .map_err(serde::de::Error::custom),
            "OperatorStationIdentifierType" => serde_json::from_value(value)
                .map(Self::OperatorStationIdentifierType)
                .map_err(serde::de::Error::custom),
            "RFD_ComponentIdentifierType" => serde_json::from_value(value)
                .map(Self::RfdComponentIdentifierType)
                .map_err(serde::de::Error::custom),
            _ => Err(serde::de::Error::custom(format!(
                "illegal derived type {tag}"
            ))),
        }
    }
}

/// A value in a `ID_Type` element position, including legal derived types.
#[derive(Debug, Clone, PartialEq)]
pub enum IdTypeValue {
    /// Carry `ID_Type`.
    IdType(IdType),
    /// Carry `ACTDF_TaskID_Type`.
    ActdfTaskIdType(ActdfTaskIdType),
    /// Carry `AccessAssessmentID_Type`.
    AccessAssessmentIdType(AccessAssessmentIdType),
    /// Carry `ActionID_Type`.
    ActionIdType(ActionIdType),
    /// Carry `ActionPlanCommandID_Type`.
    ActionPlanCommandIdType(ActionPlanCommandIdType),
    /// Carry `ActionPlanID_Type`.
    ActionPlanIdType(ActionPlanIdType),
    /// Carry `ActionPlanValidationID_Type`.
    ActionPlanValidationIdType(ActionPlanValidationIdType),
    /// Carry `ActivityID_Type`.
    ActivityIdType(ActivityIdType),
    /// Carry `ActivityMetricsID_Type`.
    ActivityMetricsIdType(ActivityMetricsIdType),
    /// Carry `ActivityPlanCommandID_Type`.
    ActivityPlanCommandIdType(ActivityPlanCommandIdType),
    /// Carry `ActivityPlanID_Type`.
    ActivityPlanIdType(ActivityPlanIdType),
    /// Carry `ActivityPlanValidationID_Type`.
    ActivityPlanValidationIdType(ActivityPlanValidationIdType),
    /// Carry `AirCharacteristicsID_Type`.
    AirCharacteristicsIdType(AirCharacteristicsIdType),
    /// Carry `AirRecordID_Type`.
    AirRecordIdType(AirRecordIdType),
    /// Carry `AirReportID_Type`.
    AirReportIdType(AirReportIdType),
    /// Carry `AirfieldID_Type`.
    AirfieldIdType(AirfieldIdType),
    /// Carry `AirfieldReportID_Type`.
    AirfieldReportIdType(AirfieldReportIdType),
    /// Carry `AnalysisRouteID_Type`.
    AnalysisRouteIdType(AnalysisRouteIdType),
    /// Carry `AntennaID_Type`.
    AntennaIdType(AntennaIdType),
    /// Carry `AntennaResourceID_Type`.
    AntennaResourceIdType(AntennaResourceIdType),
    /// Carry `ApprovalAuthorityID_Type`.
    ApprovalAuthorityIdType(ApprovalAuthorityIdType),
    /// Carry `ApprovalPolicyID_Type`.
    ApprovalPolicyIdType(ApprovalPolicyIdType),
    /// Carry `AssessmentID_Type`.
    AssessmentIdType(AssessmentIdType),
    /// Carry `AuthorizationID_Type`.
    AuthorizationIdType(AuthorizationIdType),
    /// Carry `AutonomousPlanningActionID_Type`.
    AutonomousPlanningActionIdType(AutonomousPlanningActionIdType),
    /// Carry `BIT_ID_Type`.
    BitIdType(BitIdType),
    /// Carry `BeamID_Type`.
    BeamIdType(BeamIdType),
    /// Carry `CalibrationID_Type`.
    CalibrationIdType(CalibrationIdType),
    /// Carry `CapabilityConfigurationID_Type`.
    CapabilityConfigurationIdType(CapabilityConfigurationIdType),
    /// Carry `CapabilityCoverageAreaID_Type`.
    CapabilityCoverageAreaIdType(CapabilityCoverageAreaIdType),
    /// Carry `CapabilityCoverageAreaProcessID_Type`.
    CapabilityCoverageAreaProcessIdType(CapabilityCoverageAreaProcessIdType),
    /// Carry `CapabilityID_Type`.
    CapabilityIdType(CapabilityIdType),
    /// Carry `CargoID_Type`.
    CargoIdType(CargoIdType),
    /// Carry `CommConfigurationID_Type`.
    CommConfigurationIdType(CommConfigurationIdType),
    /// Carry `CommEffectivityID_Type`.
    CommEffectivityIdType(CommEffectivityIdType),
    /// Carry `CommPointingID_Type`.
    CommPointingIdType(CommPointingIdType),
    /// Carry `CommProfileID_Type`.
    CommProfileIdType(CommProfileIdType),
    /// Carry `CommSupportID_Type`.
    CommSupportIdType(CommSupportIdType),
    /// Carry `CommSupportPlanCommandID_Type`.
    CommSupportPlanCommandIdType(CommSupportPlanCommandIdType),
    /// Carry `CommSupportPlanID_Type`.
    CommSupportPlanIdType(CommSupportPlanIdType),
    /// Carry `CommSupportPlanValidationID_Type`.
    CommSupportPlanValidationIdType(CommSupportPlanValidationIdType),
    /// Carry `CommTerminalPlanID_Type`.
    CommTerminalPlanIdType(CommTerminalPlanIdType),
    /// Carry `CommUserIO_ID_Type`.
    CommUserIoIdType(CommUserIoIdType),
    /// Carry `CommUserLinkID_Type`.
    CommUserLinkIdType(CommUserLinkIdType),
    /// Carry `CommWindowID_Type`.
    CommWindowIdType(CommWindowIdType),
    /// Carry `CommandID_Type`.
    CommandIdType(CommandIdType),
    /// Carry `ComponentConfigurationID_Type`.
    ComponentConfigurationIdType(ComponentConfigurationIdType),
    /// Carry `ComponentElementID_Type`.
    ComponentElementIdType(ComponentElementIdType),
    /// Carry `ComponentID_Type`.
    ComponentIdType(ComponentIdType),
    /// Carry `ConflictID_Type`.
    ConflictIdType(ConflictIdType),
    /// Carry `CoordinatedRequirementSetID_Type`.
    CoordinatedRequirementSetIdType(CoordinatedRequirementSetIdType),
    /// Carry `CorrelatedEntityID_Type`.
    CorrelatedEntityIdType(CorrelatedEntityIdType),
    /// Carry `CounterSpaceDeconflictionID_Type`.
    CounterSpaceDeconflictionIdType(CounterSpaceDeconflictionIdType),
    /// Carry `CounterSpaceReportID_Type`.
    CounterSpaceReportIdType(CounterSpaceReportIdType),
    /// Carry `CryptoPortID_Type`.
    CryptoPortIdType(CryptoPortIdType),
    /// Carry `CryptoSlotGroupID_Type`.
    CryptoSlotGroupIdType(CryptoSlotGroupIdType),
    /// Carry `DLZ_ID_Type`.
    DlzIdType(DlzIdType),
    /// Carry `DMPI_DesignationID_Type`.
    DmpiDesignationIdType(DmpiDesignationIdType),
    /// Carry `DMPI_ID_Type`.
    DmpiIdType(DmpiIdType),
    /// Carry `DMPI_PatternID_Type`.
    DmpiPatternIdType(DmpiPatternIdType),
    /// Carry `DamageAssessmentID_Type`.
    DamageAssessmentIdType(DamageAssessmentIdType),
    /// Carry `DamageEstimateID_Type`.
    DamageEstimateIdType(DamageEstimateIdType),
    /// Carry `DamageReportID_Type`.
    DamageReportIdType(DamageReportIdType),
    /// Carry `DamageSummaryID_Type`.
    DamageSummaryIdType(DamageSummaryIdType),
    /// Carry `DataDestinationID_Type`.
    DataDestinationIdType(DataDestinationIdType),
    /// Carry `DataPlanID_Type`.
    DataPlanIdType(DataPlanIdType),
    /// Carry `DataRecordInstanceID_Type`.
    DataRecordInstanceIdType(DataRecordInstanceIdType),
    /// Carry `DataRecordListID_Type`.
    DataRecordListIdType(DataRecordListIdType),
    /// Carry `DesignationID_Type`.
    DesignationIdType(DesignationIdType),
    /// Carry `DwellID_Type`.
    DwellIdType(DwellIdType),
    /// Carry `EEI_ID_Type`.
    EeiIdType(EeiIdType),
    /// Carry `EW_ConfigurationID_Type`.
    EwConfigurationIdType(EwConfigurationIdType),
    /// Carry `EarthOrientationParametersID_Type`.
    EarthOrientationParametersIdType(EarthOrientationParametersIdType),
    /// Carry `EffectID_Type`.
    EffectIdType(EffectIdType),
    /// Carry `EffectPlanCommandID_Type`.
    EffectPlanCommandIdType(EffectPlanCommandIdType),
    /// Carry `EffectPlanID_Type`.
    EffectPlanIdType(EffectPlanIdType),
    /// Carry `EffectPlanValidationID_Type`.
    EffectPlanValidationIdType(EffectPlanValidationIdType),
    /// Carry `EmitterModeID_Type`.
    EmitterModeIdType(EmitterModeIdType),
    /// Carry `EmitterRecordID_Type`.
    EmitterRecordIdType(EmitterRecordIdType),
    /// Carry `EntityID_Type`.
    EntityIdType(EntityIdType),
    /// Carry `EntityOrbitalCSO_ID_Type`.
    EntityOrbitalCsoIdType(EntityOrbitalCsoIdType),
    /// Carry `EntityOrbitalElementSetID_Type`.
    EntityOrbitalElementSetIdType(EntityOrbitalElementSetIdType),
    /// Carry `EntityOrbitalEphemerisID_Type`.
    EntityOrbitalEphemerisIdType(EntityOrbitalEphemerisIdType),
    /// Carry `EntityOrbitalVCM_ID_Type`.
    EntityOrbitalVcmIdType(EntityOrbitalVcmIdType),
    /// Carry `ExecutionPlanSetID_Type`.
    ExecutionPlanSetIdType(ExecutionPlanSetIdType),
    /// Carry `ExternalRequestedDataID_Type`.
    ExternalRequestedDataIdType(ExternalRequestedDataIdType),
    /// Carry `FacilityCharacteristicsID_Type`.
    FacilityCharacteristicsIdType(FacilityCharacteristicsIdType),
    /// Carry `FacilityRecordID_Type`.
    FacilityRecordIdType(FacilityRecordIdType),
    /// Carry `FaultID_Type`.
    FaultIdType(FaultIdType),
    /// Carry `FileLocationID_Type`.
    FileLocationIdType(FileLocationIdType),
    /// Carry `FileMetadataID_Type`.
    FileMetadataIdType(FileMetadataIdType),
    /// Carry `FilterID_Type`.
    FilterIdType(FilterIdType),
    /// Carry `FlightRuleID_Type`.
    FlightRuleIdType(FlightRuleIdType),
    /// Carry `ForeignKeyMapID_Type`.
    ForeignKeyMapIdType(ForeignKeyMapIdType),
    /// Carry `ForeignKeyPairID_Type`.
    ForeignKeyPairIdType(ForeignKeyPairIdType),
    /// Carry `FunctionID_Type`.
    FunctionIdType(FunctionIdType),
    /// Carry `GimbalAxisID_Type`.
    GimbalAxisIdType(GimbalAxisIdType),
    /// Carry `IFF_MeasurementAndDataMessageID_Type`.
    IffMeasurementAndDataMessageIdType(IffMeasurementAndDataMessageIdType),
    /// Carry `IFF_MeasurementAndDataReportMessageID_Type`.
    IffMeasurementAndDataReportMessageIdType(IffMeasurementAndDataReportMessageIdType),
    /// Carry `IFF_MeasurementID_Type`.
    IffMeasurementIdType(IffMeasurementIdType),
    /// Carry `JamStrobeReportID_Type`.
    JamStrobeReportIdType(JamStrobeReportIdType),
    /// Carry `LAR_ID_Type`.
    LarIdType(LarIdType),
    /// Carry `LandCharacteristicsID_Type`.
    LandCharacteristicsIdType(LandCharacteristicsIdType),
    /// Carry `LandRecordID_Type`.
    LandRecordIdType(LandRecordIdType),
    /// Carry `LaunchObjectID_Type`.
    LaunchObjectIdType(LaunchObjectIdType),
    /// Carry `LaunchObservationID_Type`.
    LaunchObservationIdType(LaunchObservationIdType),
    /// Carry `MDF_ID_Type`.
    MdfIdType(MdfIdType),
    /// Carry `MDF_SubCategoryFileID_Type`.
    MdfSubCategoryFileIdType(MdfSubCategoryFileIdType),
    /// Carry `MDF_SubCategoryID_Type`.
    MdfSubCategoryIdType(MdfSubCategoryIdType),
    /// Carry `MeasurementGroupID_Type`.
    MeasurementGroupIdType(MeasurementGroupIdType),
    /// Carry `MeasurementID_Type`.
    MeasurementIdType(MeasurementIdType),
    /// Carry `MessageConfigurationID_Type`.
    MessageConfigurationIdType(MessageConfigurationIdType),
    /// Carry `MessageTransmissionFilterRecordID_Type`.
    MessageTransmissionFilterRecordIdType(MessageTransmissionFilterRecordIdType),
    /// Carry `MessageTransmissionListID_Type`.
    MessageTransmissionListIdType(MessageTransmissionListIdType),
    /// Carry `MetadataID_Type`.
    MetadataIdType(MetadataIdType),
    /// Carry `MissileCharacteristicsID_Type`.
    MissileCharacteristicsIdType(MissileCharacteristicsIdType),
    /// Carry `MissileRecordID_Type`.
    MissileRecordIdType(MissileRecordIdType),
    /// Carry `MissionContingencyAlertID_Type`.
    MissionContingencyAlertIdType(MissionContingencyAlertIdType),
    /// Carry `MissionEnvironmentOverrideID_Type`.
    MissionEnvironmentOverrideIdType(MissionEnvironmentOverrideIdType),
    /// Carry `MissionID_Type`.
    MissionIdType(MissionIdType),
    /// Carry `MissionPlanActivationCommandID_Type`.
    MissionPlanActivationCommandIdType(MissionPlanActivationCommandIdType),
    /// Carry `MissionPlanCommandID_Type`.
    MissionPlanCommandIdType(MissionPlanCommandIdType),
    /// Carry `MissionPlanID_Type`.
    MissionPlanIdType(MissionPlanIdType),
    /// Carry `MissionPlanMetricsReportID_Type`.
    MissionPlanMetricsReportIdType(MissionPlanMetricsReportIdType),
    /// Carry `MissionPlanValidationID_Type`.
    MissionPlanValidationIdType(MissionPlanValidationIdType),
    /// Carry `MissionPlanningOrchestrationID_Type`.
    MissionPlanningOrchestrationIdType(MissionPlanningOrchestrationIdType),
    /// Carry `MissionValidationID_Type`.
    MissionValidationIdType(MissionValidationIdType),
    /// Carry `MultistaticEmitterDataID_Type`.
    MultistaticEmitterDataIdType(MultistaticEmitterDataIdType),
    /// Carry `MultistaticEmitterID_Type`.
    MultistaticEmitterIdType(MultistaticEmitterIdType),
    /// Carry `MvAnomalyEventID_Type`.
    MvAnomalyEventIdType(MvAnomalyEventIdType),
    /// Carry `NetworkLinkID_Type`.
    NetworkLinkIdType(NetworkLinkIdType),
    /// Carry `NotificationID_Type`.
    NotificationIdType(NotificationIdType),
    /// Carry `OB_CorrelationRecordID_Type`.
    ObCorrelationRecordIdType(ObCorrelationRecordIdType),
    /// Carry `ObservationMeasurementID_Type`.
    ObservationMeasurementIdType(ObservationMeasurementIdType),
    /// Carry `ObservationMeasurementReportID_Type`.
    ObservationMeasurementReportIdType(ObservationMeasurementReportIdType),
    /// Carry `ObservationReportID_Type`.
    ObservationReportIdType(ObservationReportIdType),
    /// Carry `OpLineID_Type`.
    OpLineIdType(OpLineIdType),
    /// Carry `OpPointID_Type`.
    OpPointIdType(OpPointIdType),
    /// Carry `OpRoutingID_Type`.
    OpRoutingIdType(OpRoutingIdType),
    /// Carry `OpVolumeID_Type`.
    OpVolumeIdType(OpVolumeIdType),
    /// Carry `OpZoneID_Type`.
    OpZoneIdType(OpZoneIdType),
    /// Carry `OperatorActionID_Type`.
    OperatorActionIdType(OperatorActionIdType),
    /// Carry `OperatorAnnotationID_Type`.
    OperatorAnnotationIdType(OperatorAnnotationIdType),
    /// Carry `OperatorID_Type`.
    OperatorIdType(OperatorIdType),
    /// Carry `OperatorLocationOfInterestID_Type`.
    OperatorLocationOfInterestIdType(OperatorLocationOfInterestIdType),
    /// Carry `OperatorNotificationAckID_Type`.
    OperatorNotificationAckIdType(OperatorNotificationAckIdType),
    /// Carry `OperatorNotificationID_Type`.
    OperatorNotificationIdType(OperatorNotificationIdType),
    /// Carry `OperatorNotificationOverrideID_Type`.
    OperatorNotificationOverrideIdType(OperatorNotificationOverrideIdType),
    /// Carry `OperatorRoleID_Type`.
    OperatorRoleIdType(OperatorRoleIdType),
    /// Carry `OrbitActivityPlanCommandID_Type`.
    OrbitActivityPlanCommandIdType(OrbitActivityPlanCommandIdType),
    /// Carry `OrbitActivityPlanID_Type`.
    OrbitActivityPlanIdType(OrbitActivityPlanIdType),
    /// Carry `OrbitActivityPlanValidationID_Type`.
    OrbitActivityPlanValidationIdType(OrbitActivityPlanValidationIdType),
    /// Carry `OrbitConjunctionAssessmentID_Type`.
    OrbitConjunctionAssessmentIdType(OrbitConjunctionAssessmentIdType),
    /// Carry `OrbitKinematicsSequenceID_Type`.
    OrbitKinematicsSequenceIdType(OrbitKinematicsSequenceIdType),
    /// Carry `OrbitManeuverSegmentID_Type`.
    OrbitManeuverSegmentIdType(OrbitManeuverSegmentIdType),
    /// Carry `OrbitMetricsID_Type`.
    OrbitMetricsIdType(OrbitMetricsIdType),
    /// Carry `OrbitPlanCommandID_Type`.
    OrbitPlanCommandIdType(OrbitPlanCommandIdType),
    /// Carry `OrbitPlanID_Type`.
    OrbitPlanIdType(OrbitPlanIdType),
    /// Carry `OrbitPlanValidationID_Type`.
    OrbitPlanValidationIdType(OrbitPlanValidationIdType),
    /// Carry `OrbitalConjunctionID_Type`.
    OrbitalConjunctionIdType(OrbitalConjunctionIdType),
    /// Carry `OrbitalDebrisCloudID_Type`.
    OrbitalDebrisCloudIdType(OrbitalDebrisCloudIdType),
    /// Carry `OrbitalDebrisEstimateID_Type`.
    OrbitalDebrisEstimateIdType(OrbitalDebrisEstimateIdType),
    /// Carry `OrbitalDebrisID_Type`.
    OrbitalDebrisIdType(OrbitalDebrisIdType),
    /// Carry `OrbitalManeuverID_Type`.
    OrbitalManeuverIdType(OrbitalManeuverIdType),
    /// Carry `OrderOfBattleID_Type`.
    OrderOfBattleIdType(OrderOfBattleIdType),
    /// Carry `OrdersMetadataID_Type`.
    OrdersMetadataIdType(OrdersMetadataIdType),
    /// Carry `PO_ApertureID_Type`.
    PoApertureIdType(PoApertureIdType),
    /// Carry `PO_FPA_ChannelID_Type`.
    PoFpaChannelIdType(PoFpaChannelIdType),
    /// Carry `PO_FPA_ID_Type`.
    PoFpaIdType(PoFpaIdType),
    /// Carry `PO_FilterID_Type`.
    PoFilterIdType(PoFilterIdType),
    /// Carry `PO_LensID_Type`.
    PoLensIdType(PoLensIdType),
    /// Carry `PO_ProcessingStageID_Type`.
    PoProcessingStageIdType(PoProcessingStageIdType),
    /// Carry `PO_ProductGeneratorID_Type`.
    PoProductGeneratorIdType(PoProductGeneratorIdType),
    /// Carry `PO_ProductGeneratorOutputID_Type`.
    PoProductGeneratorOutputIdType(PoProductGeneratorOutputIdType),
    /// Carry `PO_WindowID_Type`.
    PoWindowIdType(PoWindowIdType),
    /// Carry `PackageID_Type`.
    PackageIdType(PackageIdType),
    /// Carry `PackageReadinessID_Type`.
    PackageReadinessIdType(PackageReadinessIdType),
    /// Carry `ParameterID_Type`.
    ParameterIdType(ParameterIdType),
    /// Carry `PathID_Type`.
    PathIdType(PathIdType),
    /// Carry `PlanScoresID_Type`.
    PlanScoresIdType(PlanScoresIdType),
    /// Carry `PlannedActivityEventID_Type`.
    PlannedActivityEventIdType(PlannedActivityEventIdType),
    /// Carry `PlannedActivityID_Type`.
    PlannedActivityIdType(PlannedActivityIdType),
    /// Carry `PlanningFunctionID_Type`.
    PlanningFunctionIdType(PlanningFunctionIdType),
    /// Carry `PlanningProcessID_Type`.
    PlanningProcessIdType(PlanningProcessIdType),
    /// Carry `PlatformRecordID_Type`.
    PlatformRecordIdType(PlatformRecordIdType),
    /// Carry `PlatformReportID_Type`.
    PlatformReportIdType(PlatformReportIdType),
    /// Carry `PrioritizationListID_Type`.
    PrioritizationListIdType(PrioritizationListIdType),
    /// Carry `ProductDownloadPlanID_Type`.
    ProductDownloadPlanIdType(ProductDownloadPlanIdType),
    /// Carry `ProductDownloadReportID_Type`.
    ProductDownloadReportIdType(ProductDownloadReportIdType),
    /// Carry `ProductLocationID_Type`.
    ProductLocationIdType(ProductLocationIdType),
    /// Carry `ProductMetadataID_Type`.
    ProductMetadataIdType(ProductMetadataIdType),
    /// Carry `ProductOrFileClassificationPlanID_Type`.
    ProductOrFileClassificationPlanIdType(ProductOrFileClassificationPlanIdType),
    /// Carry `ProductOrFileClassificationReportID_Type`.
    ProductOrFileClassificationReportIdType(ProductOrFileClassificationReportIdType),
    /// Carry `ProductOrFileDisseminationConfigurationID_Type`.
    ProductOrFileDisseminationConfigurationIdType(ProductOrFileDisseminationConfigurationIdType),
    /// Carry `ProductOrFileDisseminationDestinationID_Type`.
    ProductOrFileDisseminationDestinationIdType(ProductOrFileDisseminationDestinationIdType),
    /// Carry `ProductOrFileDisseminationPlanID_Type`.
    ProductOrFileDisseminationPlanIdType(ProductOrFileDisseminationPlanIdType),
    /// Carry `ProductOrFileDisseminationReportID_Type`.
    ProductOrFileDisseminationReportIdType(ProductOrFileDisseminationReportIdType),
    /// Carry `ProductProcessingPlanID_Type`.
    ProductProcessingPlanIdType(ProductProcessingPlanIdType),
    /// Carry `ProductProcessingReportID_Type`.
    ProductProcessingReportIdType(ProductProcessingReportIdType),
    /// Carry `PropagatorSettingsID_Type`.
    PropagatorSettingsIdType(PropagatorSettingsIdType),
    /// Carry `PulseDataID_Type`.
    PulseDataIdType(PulseDataIdType),
    /// Carry `RDMA_InitializeID_Type`.
    RdmaInitializeIdType(RdmaInitializeIdType),
    /// Carry `RDMA_InitializeSetupID_Type`.
    RdmaInitializeSetupIdType(RdmaInitializeSetupIdType),
    /// Carry `RF_ProfileID_Type`.
    RfProfileIdType(RfProfileIdType),
    /// Carry `RF_ReportID_Type`.
    RfReportIdType(RfReportIdType),
    /// Carry `RF_ResourceAllocationID_Type`.
    RfResourceAllocationIdType(RfResourceAllocationIdType),
    /// Carry `RadarAltimeterReportID_Type`.
    RadarAltimeterReportIdType(RadarAltimeterReportIdType),
    /// Carry `RecordID_Type`.
    RecordIdType(RecordIdType),
    /// Carry `ReferenceFrameID_Type`.
    ReferenceFrameIdType(ReferenceFrameIdType),
    /// Carry `RelationshipDesignationID_Type`.
    RelationshipDesignationIdType(RelationshipDesignationIdType),
    /// Carry `RequestID_Type`.
    RequestIdType(RequestIdType),
    /// Carry `RequirementAllocationWindowID_Type`.
    RequirementAllocationWindowIdType(RequirementAllocationWindowIdType),
    /// Carry `RequirementMetricsID_Type`.
    RequirementMetricsIdType(RequirementMetricsIdType),
    /// Carry `RequirementOptionsID_Type`.
    RequirementOptionsIdType(RequirementOptionsIdType),
    /// Carry `RequirementSetID_Type`.
    RequirementSetIdType(RequirementSetIdType),
    /// Carry `RequirementsTemplateID_Type`.
    RequirementsTemplateIdType(RequirementsTemplateIdType),
    /// Carry `ResponseID_Type`.
    ResponseIdType(ResponseIdType),
    /// Carry `ResponsePlanCommandID_Type`.
    ResponsePlanCommandIdType(ResponsePlanCommandIdType),
    /// Carry `ResponsePlanID_Type`.
    ResponsePlanIdType(ResponsePlanIdType),
    /// Carry `ResponsePlanValidationID_Type`.
    ResponsePlanValidationIdType(ResponsePlanValidationIdType),
    /// Carry `RouteActivityPlanCommandID_Type`.
    RouteActivityPlanCommandIdType(RouteActivityPlanCommandIdType),
    /// Carry `RouteActivityPlanID_Type`.
    RouteActivityPlanIdType(RouteActivityPlanIdType),
    /// Carry `RouteActivityPlanValidationID_Type`.
    RouteActivityPlanValidationIdType(RouteActivityPlanValidationIdType),
    /// Carry `RouteMetricsID_Type`.
    RouteMetricsIdType(RouteMetricsIdType),
    /// Carry `RoutePlanCommandID_Type`.
    RoutePlanCommandIdType(RoutePlanCommandIdType),
    /// Carry `RoutePlanID_Type`.
    RoutePlanIdType(RoutePlanIdType),
    /// Carry `RoutePlanValidationID_Type`.
    RoutePlanValidationIdType(RoutePlanValidationIdType),
    /// Carry `RunwayID_Type`.
    RunwayIdType(RunwayIdType),
    /// Carry `SatellitePhysicalConfigurationID_Type`.
    SatellitePhysicalConfigurationIdType(SatellitePhysicalConfigurationIdType),
    /// Carry `ScoringProcessID_Type`.
    ScoringProcessIdType(ScoringProcessIdType),
    /// Carry `SeaSubsurfaceCharacteristicsID_Type`.
    SeaSubsurfaceCharacteristicsIdType(SeaSubsurfaceCharacteristicsIdType),
    /// Carry `SeaSubsurfaceRecordID_Type`.
    SeaSubsurfaceRecordIdType(SeaSubsurfaceRecordIdType),
    /// Carry `SeaSurfaceCharacteristicsID_Type`.
    SeaSurfaceCharacteristicsIdType(SeaSurfaceCharacteristicsIdType),
    /// Carry `SeaSurfaceRecordID_Type`.
    SeaSurfaceRecordIdType(SeaSurfaceRecordIdType),
    /// Carry `SegmentID_Type`.
    SegmentIdType(SegmentIdType),
    /// Carry `SensorID_Type`.
    SensorIdType(SensorIdType),
    /// Carry `ServiceErrorReportID_Type`.
    ServiceErrorReportIdType(ServiceErrorReportIdType),
    /// Carry `ServiceID_Type`.
    ServiceIdType(ServiceIdType),
    /// Carry `SignalID_Type`.
    SignalIdType(SignalIdType),
    /// Carry `SignalReportID_Type`.
    SignalReportIdType(SignalReportIdType),
    /// Carry `SpaceCharacteristicsID_Type`.
    SpaceCharacteristicsIdType(SpaceCharacteristicsIdType),
    /// Carry `SpaceRecordID_Type`.
    SpaceRecordIdType(SpaceRecordIdType),
    /// Carry `SpaceWeatherID_Type`.
    SpaceWeatherIdType(SpaceWeatherIdType),
    /// Carry `SpectralDensityReportID_Type`.
    SpectralDensityReportIdType(SpectralDensityReportIdType),
    /// Carry `StoreLoadoutConfigurationID_Type`.
    StoreLoadoutConfigurationIdType(StoreLoadoutConfigurationIdType),
    /// Carry `StoreManagementReportID_Type`.
    StoreManagementReportIdType(StoreManagementReportIdType),
    /// Carry `SubCapabilityID_Type`.
    SubCapabilityIdType(SubCapabilityIdType),
    /// Carry `SubsystemBIT_ConfigurationID_Type`.
    SubsystemBitConfigurationIdType(SubsystemBitConfigurationIdType),
    /// Carry `SubsystemCalibrationConfigurationID_Type`.
    SubsystemCalibrationConfigurationIdType(SubsystemCalibrationConfigurationIdType),
    /// Carry `SubsystemConfigurationID_Type`.
    SubsystemConfigurationIdType(SubsystemConfigurationIdType),
    /// Carry `SubsystemID_Type`.
    SubsystemIdType(SubsystemIdType),
    /// Carry `SubsystemMaintenanceConfigurationID_Type`.
    SubsystemMaintenanceConfigurationIdType(SubsystemMaintenanceConfigurationIdType),
    /// Carry `SubsystemMaintenanceID_Type`.
    SubsystemMaintenanceIdType(SubsystemMaintenanceIdType),
    /// Carry `SupportCapabilityID_Type`.
    SupportCapabilityIdType(SupportCapabilityIdType),
    /// Carry `SupportPlanID_Type`.
    SupportPlanIdType(SupportPlanIdType),
    /// Carry `SupportedResolutionID_Type`.
    SupportedResolutionIdType(SupportedResolutionIdType),
    /// Carry `SurvivabilityRiskLevelID_Type`.
    SurvivabilityRiskLevelIdType(SurvivabilityRiskLevelIdType),
    /// Carry `SystemID_Type`.
    SystemIdType(SystemIdType),
    /// Carry `SystemOrbitalElementSetID_Type`.
    SystemOrbitalElementSetIdType(SystemOrbitalElementSetIdType),
    /// Carry `SystemOrbitalEphemerisID_Type`.
    SystemOrbitalEphemerisIdType(SystemOrbitalEphemerisIdType),
    /// Carry `SystemOrbitalVCM_ID_Type`.
    SystemOrbitalVcmIdType(SystemOrbitalVcmIdType),
    /// Carry `SystemReadinessID_Type`.
    SystemReadinessIdType(SystemReadinessIdType),
    /// Carry `TagAssociationID_Type`.
    TagAssociationIdType(TagAssociationIdType),
    /// Carry `TaskID_Type`.
    TaskIdType(TaskIdType),
    /// Carry `TaskPlanCommandID_Type`.
    TaskPlanCommandIdType(TaskPlanCommandIdType),
    /// Carry `TaskPlanID_Type`.
    TaskPlanIdType(TaskPlanIdType),
    /// Carry `TaskPlanValidationID_Type`.
    TaskPlanValidationIdType(TaskPlanValidationIdType),
    /// Carry `TrackOptionsID_Type`.
    TrackOptionsIdType(TrackOptionsIdType),
    /// Carry `TurretReportID_Type`.
    TurretReportIdType(TurretReportIdType),
    /// Carry `UMID_AudioID_Type`.
    UmidAudioIdType(UmidAudioIdType),
    /// Carry `UMID_DataID_Type`.
    UmidDataIdType(UmidDataIdType),
    /// Carry `UMID_SystemID_Type`.
    UmidSystemIdType(UmidSystemIdType),
    /// Carry `UMID_VideoID_Type`.
    UmidVideoIdType(UmidVideoIdType),
    /// Carry `UnitID_Type`.
    UnitIdType(UnitIdType),
    /// Carry `UnitRecordID_Type`.
    UnitRecordIdType(UnitRecordIdType),
    /// Carry `VersionedID_Type`.
    VersionedIdType(VersionedIdType),
    /// Carry `WeatherDatasetID_Type`.
    WeatherDatasetIdType(WeatherDatasetIdType),
}
impl Serialize for IdTypeValue {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let (mut value, tag) = match self {
            Self::IdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ID_Type",
            ),
            Self::ActdfTaskIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ACTDF_TaskID_Type",
            ),
            Self::AccessAssessmentIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "AccessAssessmentID_Type",
            ),
            Self::ActionIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ActionID_Type",
            ),
            Self::ActionPlanCommandIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ActionPlanCommandID_Type",
            ),
            Self::ActionPlanIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ActionPlanID_Type",
            ),
            Self::ActionPlanValidationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ActionPlanValidationID_Type",
            ),
            Self::ActivityIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ActivityID_Type",
            ),
            Self::ActivityMetricsIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ActivityMetricsID_Type",
            ),
            Self::ActivityPlanCommandIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ActivityPlanCommandID_Type",
            ),
            Self::ActivityPlanIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ActivityPlanID_Type",
            ),
            Self::ActivityPlanValidationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ActivityPlanValidationID_Type",
            ),
            Self::AirCharacteristicsIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "AirCharacteristicsID_Type",
            ),
            Self::AirRecordIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "AirRecordID_Type",
            ),
            Self::AirReportIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "AirReportID_Type",
            ),
            Self::AirfieldIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "AirfieldID_Type",
            ),
            Self::AirfieldReportIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "AirfieldReportID_Type",
            ),
            Self::AnalysisRouteIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "AnalysisRouteID_Type",
            ),
            Self::AntennaIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "AntennaID_Type",
            ),
            Self::AntennaResourceIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "AntennaResourceID_Type",
            ),
            Self::ApprovalAuthorityIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ApprovalAuthorityID_Type",
            ),
            Self::ApprovalPolicyIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ApprovalPolicyID_Type",
            ),
            Self::AssessmentIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "AssessmentID_Type",
            ),
            Self::AuthorizationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "AuthorizationID_Type",
            ),
            Self::AutonomousPlanningActionIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "AutonomousPlanningActionID_Type",
            ),
            Self::BitIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "BIT_ID_Type",
            ),
            Self::BeamIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "BeamID_Type",
            ),
            Self::CalibrationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "CalibrationID_Type",
            ),
            Self::CapabilityConfigurationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "CapabilityConfigurationID_Type",
            ),
            Self::CapabilityCoverageAreaIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "CapabilityCoverageAreaID_Type",
            ),
            Self::CapabilityCoverageAreaProcessIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "CapabilityCoverageAreaProcessID_Type",
            ),
            Self::CapabilityIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "CapabilityID_Type",
            ),
            Self::CargoIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "CargoID_Type",
            ),
            Self::CommConfigurationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "CommConfigurationID_Type",
            ),
            Self::CommEffectivityIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "CommEffectivityID_Type",
            ),
            Self::CommPointingIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "CommPointingID_Type",
            ),
            Self::CommProfileIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "CommProfileID_Type",
            ),
            Self::CommSupportIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "CommSupportID_Type",
            ),
            Self::CommSupportPlanCommandIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "CommSupportPlanCommandID_Type",
            ),
            Self::CommSupportPlanIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "CommSupportPlanID_Type",
            ),
            Self::CommSupportPlanValidationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "CommSupportPlanValidationID_Type",
            ),
            Self::CommTerminalPlanIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "CommTerminalPlanID_Type",
            ),
            Self::CommUserIoIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "CommUserIO_ID_Type",
            ),
            Self::CommUserLinkIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "CommUserLinkID_Type",
            ),
            Self::CommWindowIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "CommWindowID_Type",
            ),
            Self::CommandIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "CommandID_Type",
            ),
            Self::ComponentConfigurationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ComponentConfigurationID_Type",
            ),
            Self::ComponentElementIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ComponentElementID_Type",
            ),
            Self::ComponentIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ComponentID_Type",
            ),
            Self::ConflictIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ConflictID_Type",
            ),
            Self::CoordinatedRequirementSetIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "CoordinatedRequirementSetID_Type",
            ),
            Self::CorrelatedEntityIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "CorrelatedEntityID_Type",
            ),
            Self::CounterSpaceDeconflictionIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "CounterSpaceDeconflictionID_Type",
            ),
            Self::CounterSpaceReportIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "CounterSpaceReportID_Type",
            ),
            Self::CryptoPortIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "CryptoPortID_Type",
            ),
            Self::CryptoSlotGroupIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "CryptoSlotGroupID_Type",
            ),
            Self::DlzIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "DLZ_ID_Type",
            ),
            Self::DmpiDesignationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "DMPI_DesignationID_Type",
            ),
            Self::DmpiIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "DMPI_ID_Type",
            ),
            Self::DmpiPatternIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "DMPI_PatternID_Type",
            ),
            Self::DamageAssessmentIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "DamageAssessmentID_Type",
            ),
            Self::DamageEstimateIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "DamageEstimateID_Type",
            ),
            Self::DamageReportIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "DamageReportID_Type",
            ),
            Self::DamageSummaryIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "DamageSummaryID_Type",
            ),
            Self::DataDestinationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "DataDestinationID_Type",
            ),
            Self::DataPlanIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "DataPlanID_Type",
            ),
            Self::DataRecordInstanceIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "DataRecordInstanceID_Type",
            ),
            Self::DataRecordListIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "DataRecordListID_Type",
            ),
            Self::DesignationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "DesignationID_Type",
            ),
            Self::DwellIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "DwellID_Type",
            ),
            Self::EeiIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "EEI_ID_Type",
            ),
            Self::EwConfigurationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "EW_ConfigurationID_Type",
            ),
            Self::EarthOrientationParametersIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "EarthOrientationParametersID_Type",
            ),
            Self::EffectIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "EffectID_Type",
            ),
            Self::EffectPlanCommandIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "EffectPlanCommandID_Type",
            ),
            Self::EffectPlanIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "EffectPlanID_Type",
            ),
            Self::EffectPlanValidationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "EffectPlanValidationID_Type",
            ),
            Self::EmitterModeIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "EmitterModeID_Type",
            ),
            Self::EmitterRecordIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "EmitterRecordID_Type",
            ),
            Self::EntityIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "EntityID_Type",
            ),
            Self::EntityOrbitalCsoIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "EntityOrbitalCSO_ID_Type",
            ),
            Self::EntityOrbitalElementSetIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "EntityOrbitalElementSetID_Type",
            ),
            Self::EntityOrbitalEphemerisIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "EntityOrbitalEphemerisID_Type",
            ),
            Self::EntityOrbitalVcmIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "EntityOrbitalVCM_ID_Type",
            ),
            Self::ExecutionPlanSetIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ExecutionPlanSetID_Type",
            ),
            Self::ExternalRequestedDataIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ExternalRequestedDataID_Type",
            ),
            Self::FacilityCharacteristicsIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "FacilityCharacteristicsID_Type",
            ),
            Self::FacilityRecordIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "FacilityRecordID_Type",
            ),
            Self::FaultIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "FaultID_Type",
            ),
            Self::FileLocationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "FileLocationID_Type",
            ),
            Self::FileMetadataIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "FileMetadataID_Type",
            ),
            Self::FilterIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "FilterID_Type",
            ),
            Self::FlightRuleIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "FlightRuleID_Type",
            ),
            Self::ForeignKeyMapIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ForeignKeyMapID_Type",
            ),
            Self::ForeignKeyPairIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ForeignKeyPairID_Type",
            ),
            Self::FunctionIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "FunctionID_Type",
            ),
            Self::GimbalAxisIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "GimbalAxisID_Type",
            ),
            Self::IffMeasurementAndDataMessageIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "IFF_MeasurementAndDataMessageID_Type",
            ),
            Self::IffMeasurementAndDataReportMessageIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "IFF_MeasurementAndDataReportMessageID_Type",
            ),
            Self::IffMeasurementIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "IFF_MeasurementID_Type",
            ),
            Self::JamStrobeReportIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "JamStrobeReportID_Type",
            ),
            Self::LarIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "LAR_ID_Type",
            ),
            Self::LandCharacteristicsIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "LandCharacteristicsID_Type",
            ),
            Self::LandRecordIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "LandRecordID_Type",
            ),
            Self::LaunchObjectIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "LaunchObjectID_Type",
            ),
            Self::LaunchObservationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "LaunchObservationID_Type",
            ),
            Self::MdfIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "MDF_ID_Type",
            ),
            Self::MdfSubCategoryFileIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "MDF_SubCategoryFileID_Type",
            ),
            Self::MdfSubCategoryIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "MDF_SubCategoryID_Type",
            ),
            Self::MeasurementGroupIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "MeasurementGroupID_Type",
            ),
            Self::MeasurementIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "MeasurementID_Type",
            ),
            Self::MessageConfigurationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "MessageConfigurationID_Type",
            ),
            Self::MessageTransmissionFilterRecordIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "MessageTransmissionFilterRecordID_Type",
            ),
            Self::MessageTransmissionListIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "MessageTransmissionListID_Type",
            ),
            Self::MetadataIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "MetadataID_Type",
            ),
            Self::MissileCharacteristicsIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "MissileCharacteristicsID_Type",
            ),
            Self::MissileRecordIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "MissileRecordID_Type",
            ),
            Self::MissionContingencyAlertIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "MissionContingencyAlertID_Type",
            ),
            Self::MissionEnvironmentOverrideIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "MissionEnvironmentOverrideID_Type",
            ),
            Self::MissionIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "MissionID_Type",
            ),
            Self::MissionPlanActivationCommandIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "MissionPlanActivationCommandID_Type",
            ),
            Self::MissionPlanCommandIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "MissionPlanCommandID_Type",
            ),
            Self::MissionPlanIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "MissionPlanID_Type",
            ),
            Self::MissionPlanMetricsReportIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "MissionPlanMetricsReportID_Type",
            ),
            Self::MissionPlanValidationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "MissionPlanValidationID_Type",
            ),
            Self::MissionPlanningOrchestrationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "MissionPlanningOrchestrationID_Type",
            ),
            Self::MissionValidationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "MissionValidationID_Type",
            ),
            Self::MultistaticEmitterDataIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "MultistaticEmitterDataID_Type",
            ),
            Self::MultistaticEmitterIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "MultistaticEmitterID_Type",
            ),
            Self::MvAnomalyEventIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "MvAnomalyEventID_Type",
            ),
            Self::NetworkLinkIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "NetworkLinkID_Type",
            ),
            Self::NotificationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "NotificationID_Type",
            ),
            Self::ObCorrelationRecordIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "OB_CorrelationRecordID_Type",
            ),
            Self::ObservationMeasurementIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ObservationMeasurementID_Type",
            ),
            Self::ObservationMeasurementReportIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ObservationMeasurementReportID_Type",
            ),
            Self::ObservationReportIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ObservationReportID_Type",
            ),
            Self::OpLineIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "OpLineID_Type",
            ),
            Self::OpPointIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "OpPointID_Type",
            ),
            Self::OpRoutingIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "OpRoutingID_Type",
            ),
            Self::OpVolumeIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "OpVolumeID_Type",
            ),
            Self::OpZoneIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "OpZoneID_Type",
            ),
            Self::OperatorActionIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "OperatorActionID_Type",
            ),
            Self::OperatorAnnotationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "OperatorAnnotationID_Type",
            ),
            Self::OperatorIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "OperatorID_Type",
            ),
            Self::OperatorLocationOfInterestIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "OperatorLocationOfInterestID_Type",
            ),
            Self::OperatorNotificationAckIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "OperatorNotificationAckID_Type",
            ),
            Self::OperatorNotificationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "OperatorNotificationID_Type",
            ),
            Self::OperatorNotificationOverrideIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "OperatorNotificationOverrideID_Type",
            ),
            Self::OperatorRoleIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "OperatorRoleID_Type",
            ),
            Self::OrbitActivityPlanCommandIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "OrbitActivityPlanCommandID_Type",
            ),
            Self::OrbitActivityPlanIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "OrbitActivityPlanID_Type",
            ),
            Self::OrbitActivityPlanValidationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "OrbitActivityPlanValidationID_Type",
            ),
            Self::OrbitConjunctionAssessmentIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "OrbitConjunctionAssessmentID_Type",
            ),
            Self::OrbitKinematicsSequenceIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "OrbitKinematicsSequenceID_Type",
            ),
            Self::OrbitManeuverSegmentIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "OrbitManeuverSegmentID_Type",
            ),
            Self::OrbitMetricsIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "OrbitMetricsID_Type",
            ),
            Self::OrbitPlanCommandIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "OrbitPlanCommandID_Type",
            ),
            Self::OrbitPlanIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "OrbitPlanID_Type",
            ),
            Self::OrbitPlanValidationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "OrbitPlanValidationID_Type",
            ),
            Self::OrbitalConjunctionIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "OrbitalConjunctionID_Type",
            ),
            Self::OrbitalDebrisCloudIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "OrbitalDebrisCloudID_Type",
            ),
            Self::OrbitalDebrisEstimateIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "OrbitalDebrisEstimateID_Type",
            ),
            Self::OrbitalDebrisIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "OrbitalDebrisID_Type",
            ),
            Self::OrbitalManeuverIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "OrbitalManeuverID_Type",
            ),
            Self::OrderOfBattleIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "OrderOfBattleID_Type",
            ),
            Self::OrdersMetadataIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "OrdersMetadataID_Type",
            ),
            Self::PoApertureIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "PO_ApertureID_Type",
            ),
            Self::PoFpaChannelIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "PO_FPA_ChannelID_Type",
            ),
            Self::PoFpaIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "PO_FPA_ID_Type",
            ),
            Self::PoFilterIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "PO_FilterID_Type",
            ),
            Self::PoLensIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "PO_LensID_Type",
            ),
            Self::PoProcessingStageIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "PO_ProcessingStageID_Type",
            ),
            Self::PoProductGeneratorIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "PO_ProductGeneratorID_Type",
            ),
            Self::PoProductGeneratorOutputIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "PO_ProductGeneratorOutputID_Type",
            ),
            Self::PoWindowIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "PO_WindowID_Type",
            ),
            Self::PackageIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "PackageID_Type",
            ),
            Self::PackageReadinessIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "PackageReadinessID_Type",
            ),
            Self::ParameterIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ParameterID_Type",
            ),
            Self::PathIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "PathID_Type",
            ),
            Self::PlanScoresIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "PlanScoresID_Type",
            ),
            Self::PlannedActivityEventIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "PlannedActivityEventID_Type",
            ),
            Self::PlannedActivityIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "PlannedActivityID_Type",
            ),
            Self::PlanningFunctionIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "PlanningFunctionID_Type",
            ),
            Self::PlanningProcessIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "PlanningProcessID_Type",
            ),
            Self::PlatformRecordIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "PlatformRecordID_Type",
            ),
            Self::PlatformReportIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "PlatformReportID_Type",
            ),
            Self::PrioritizationListIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "PrioritizationListID_Type",
            ),
            Self::ProductDownloadPlanIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ProductDownloadPlanID_Type",
            ),
            Self::ProductDownloadReportIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ProductDownloadReportID_Type",
            ),
            Self::ProductLocationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ProductLocationID_Type",
            ),
            Self::ProductMetadataIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ProductMetadataID_Type",
            ),
            Self::ProductOrFileClassificationPlanIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ProductOrFileClassificationPlanID_Type",
            ),
            Self::ProductOrFileClassificationReportIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ProductOrFileClassificationReportID_Type",
            ),
            Self::ProductOrFileDisseminationConfigurationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ProductOrFileDisseminationConfigurationID_Type",
            ),
            Self::ProductOrFileDisseminationDestinationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ProductOrFileDisseminationDestinationID_Type",
            ),
            Self::ProductOrFileDisseminationPlanIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ProductOrFileDisseminationPlanID_Type",
            ),
            Self::ProductOrFileDisseminationReportIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ProductOrFileDisseminationReportID_Type",
            ),
            Self::ProductProcessingPlanIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ProductProcessingPlanID_Type",
            ),
            Self::ProductProcessingReportIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ProductProcessingReportID_Type",
            ),
            Self::PropagatorSettingsIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "PropagatorSettingsID_Type",
            ),
            Self::PulseDataIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "PulseDataID_Type",
            ),
            Self::RdmaInitializeIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "RDMA_InitializeID_Type",
            ),
            Self::RdmaInitializeSetupIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "RDMA_InitializeSetupID_Type",
            ),
            Self::RfProfileIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "RF_ProfileID_Type",
            ),
            Self::RfReportIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "RF_ReportID_Type",
            ),
            Self::RfResourceAllocationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "RF_ResourceAllocationID_Type",
            ),
            Self::RadarAltimeterReportIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "RadarAltimeterReportID_Type",
            ),
            Self::RecordIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "RecordID_Type",
            ),
            Self::ReferenceFrameIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ReferenceFrameID_Type",
            ),
            Self::RelationshipDesignationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "RelationshipDesignationID_Type",
            ),
            Self::RequestIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "RequestID_Type",
            ),
            Self::RequirementAllocationWindowIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "RequirementAllocationWindowID_Type",
            ),
            Self::RequirementMetricsIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "RequirementMetricsID_Type",
            ),
            Self::RequirementOptionsIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "RequirementOptionsID_Type",
            ),
            Self::RequirementSetIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "RequirementSetID_Type",
            ),
            Self::RequirementsTemplateIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "RequirementsTemplateID_Type",
            ),
            Self::ResponseIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ResponseID_Type",
            ),
            Self::ResponsePlanCommandIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ResponsePlanCommandID_Type",
            ),
            Self::ResponsePlanIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ResponsePlanID_Type",
            ),
            Self::ResponsePlanValidationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ResponsePlanValidationID_Type",
            ),
            Self::RouteActivityPlanCommandIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "RouteActivityPlanCommandID_Type",
            ),
            Self::RouteActivityPlanIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "RouteActivityPlanID_Type",
            ),
            Self::RouteActivityPlanValidationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "RouteActivityPlanValidationID_Type",
            ),
            Self::RouteMetricsIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "RouteMetricsID_Type",
            ),
            Self::RoutePlanCommandIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "RoutePlanCommandID_Type",
            ),
            Self::RoutePlanIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "RoutePlanID_Type",
            ),
            Self::RoutePlanValidationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "RoutePlanValidationID_Type",
            ),
            Self::RunwayIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "RunwayID_Type",
            ),
            Self::SatellitePhysicalConfigurationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "SatellitePhysicalConfigurationID_Type",
            ),
            Self::ScoringProcessIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ScoringProcessID_Type",
            ),
            Self::SeaSubsurfaceCharacteristicsIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "SeaSubsurfaceCharacteristicsID_Type",
            ),
            Self::SeaSubsurfaceRecordIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "SeaSubsurfaceRecordID_Type",
            ),
            Self::SeaSurfaceCharacteristicsIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "SeaSurfaceCharacteristicsID_Type",
            ),
            Self::SeaSurfaceRecordIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "SeaSurfaceRecordID_Type",
            ),
            Self::SegmentIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "SegmentID_Type",
            ),
            Self::SensorIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "SensorID_Type",
            ),
            Self::ServiceErrorReportIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ServiceErrorReportID_Type",
            ),
            Self::ServiceIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "ServiceID_Type",
            ),
            Self::SignalIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "SignalID_Type",
            ),
            Self::SignalReportIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "SignalReportID_Type",
            ),
            Self::SpaceCharacteristicsIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "SpaceCharacteristicsID_Type",
            ),
            Self::SpaceRecordIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "SpaceRecordID_Type",
            ),
            Self::SpaceWeatherIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "SpaceWeatherID_Type",
            ),
            Self::SpectralDensityReportIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "SpectralDensityReportID_Type",
            ),
            Self::StoreLoadoutConfigurationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "StoreLoadoutConfigurationID_Type",
            ),
            Self::StoreManagementReportIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "StoreManagementReportID_Type",
            ),
            Self::SubCapabilityIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "SubCapabilityID_Type",
            ),
            Self::SubsystemBitConfigurationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "SubsystemBIT_ConfigurationID_Type",
            ),
            Self::SubsystemCalibrationConfigurationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "SubsystemCalibrationConfigurationID_Type",
            ),
            Self::SubsystemConfigurationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "SubsystemConfigurationID_Type",
            ),
            Self::SubsystemIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "SubsystemID_Type",
            ),
            Self::SubsystemMaintenanceConfigurationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "SubsystemMaintenanceConfigurationID_Type",
            ),
            Self::SubsystemMaintenanceIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "SubsystemMaintenanceID_Type",
            ),
            Self::SupportCapabilityIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "SupportCapabilityID_Type",
            ),
            Self::SupportPlanIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "SupportPlanID_Type",
            ),
            Self::SupportedResolutionIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "SupportedResolutionID_Type",
            ),
            Self::SurvivabilityRiskLevelIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "SurvivabilityRiskLevelID_Type",
            ),
            Self::SystemIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "SystemID_Type",
            ),
            Self::SystemOrbitalElementSetIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "SystemOrbitalElementSetID_Type",
            ),
            Self::SystemOrbitalEphemerisIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "SystemOrbitalEphemerisID_Type",
            ),
            Self::SystemOrbitalVcmIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "SystemOrbitalVCM_ID_Type",
            ),
            Self::SystemReadinessIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "SystemReadinessID_Type",
            ),
            Self::TagAssociationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "TagAssociationID_Type",
            ),
            Self::TaskIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "TaskID_Type",
            ),
            Self::TaskPlanCommandIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "TaskPlanCommandID_Type",
            ),
            Self::TaskPlanIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "TaskPlanID_Type",
            ),
            Self::TaskPlanValidationIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "TaskPlanValidationID_Type",
            ),
            Self::TrackOptionsIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "TrackOptionsID_Type",
            ),
            Self::TurretReportIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "TurretReportID_Type",
            ),
            Self::UmidAudioIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "UMID_AudioID_Type",
            ),
            Self::UmidDataIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "UMID_DataID_Type",
            ),
            Self::UmidSystemIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "UMID_SystemID_Type",
            ),
            Self::UmidVideoIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "UMID_VideoID_Type",
            ),
            Self::UnitIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "UnitID_Type",
            ),
            Self::UnitRecordIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "UnitRecordID_Type",
            ),
            Self::VersionedIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "VersionedID_Type",
            ),
            Self::WeatherDatasetIdType(inner) => (
                serde_json::to_value(inner).map_err(serde::ser::Error::custom)?,
                "WeatherDatasetID_Type",
            ),
        };
        if tag != "ID_Type" {
            value
                .as_object_mut()
                .ok_or_else(|| serde::ser::Error::custom("expected object"))?
                .insert("$type".into(), tag.into());
        }
        value.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for IdTypeValue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let mut value = serde_json::Value::deserialize(deserializer)?;
        let tag = value
            .as_object_mut()
            .ok_or_else(|| serde::de::Error::custom("expected object"))?
            .remove("$type");
        let tag = match &tag {
            None => "ID_Type",
            Some(serde_json::Value::String(s)) => s.as_str(),
            _ => return Err(serde::de::Error::custom("$type must be a string")),
        };
        match tag {
            "ID_Type" => serde_json::from_value(value)
                .map(Self::IdType)
                .map_err(serde::de::Error::custom),
            "ACTDF_TaskID_Type" => serde_json::from_value(value)
                .map(Self::ActdfTaskIdType)
                .map_err(serde::de::Error::custom),
            "AccessAssessmentID_Type" => serde_json::from_value(value)
                .map(Self::AccessAssessmentIdType)
                .map_err(serde::de::Error::custom),
            "ActionID_Type" => serde_json::from_value(value)
                .map(Self::ActionIdType)
                .map_err(serde::de::Error::custom),
            "ActionPlanCommandID_Type" => serde_json::from_value(value)
                .map(Self::ActionPlanCommandIdType)
                .map_err(serde::de::Error::custom),
            "ActionPlanID_Type" => serde_json::from_value(value)
                .map(Self::ActionPlanIdType)
                .map_err(serde::de::Error::custom),
            "ActionPlanValidationID_Type" => serde_json::from_value(value)
                .map(Self::ActionPlanValidationIdType)
                .map_err(serde::de::Error::custom),
            "ActivityID_Type" => serde_json::from_value(value)
                .map(Self::ActivityIdType)
                .map_err(serde::de::Error::custom),
            "ActivityMetricsID_Type" => serde_json::from_value(value)
                .map(Self::ActivityMetricsIdType)
                .map_err(serde::de::Error::custom),
            "ActivityPlanCommandID_Type" => serde_json::from_value(value)
                .map(Self::ActivityPlanCommandIdType)
                .map_err(serde::de::Error::custom),
            "ActivityPlanID_Type" => serde_json::from_value(value)
                .map(Self::ActivityPlanIdType)
                .map_err(serde::de::Error::custom),
            "ActivityPlanValidationID_Type" => serde_json::from_value(value)
                .map(Self::ActivityPlanValidationIdType)
                .map_err(serde::de::Error::custom),
            "AirCharacteristicsID_Type" => serde_json::from_value(value)
                .map(Self::AirCharacteristicsIdType)
                .map_err(serde::de::Error::custom),
            "AirRecordID_Type" => serde_json::from_value(value)
                .map(Self::AirRecordIdType)
                .map_err(serde::de::Error::custom),
            "AirReportID_Type" => serde_json::from_value(value)
                .map(Self::AirReportIdType)
                .map_err(serde::de::Error::custom),
            "AirfieldID_Type" => serde_json::from_value(value)
                .map(Self::AirfieldIdType)
                .map_err(serde::de::Error::custom),
            "AirfieldReportID_Type" => serde_json::from_value(value)
                .map(Self::AirfieldReportIdType)
                .map_err(serde::de::Error::custom),
            "AnalysisRouteID_Type" => serde_json::from_value(value)
                .map(Self::AnalysisRouteIdType)
                .map_err(serde::de::Error::custom),
            "AntennaID_Type" => serde_json::from_value(value)
                .map(Self::AntennaIdType)
                .map_err(serde::de::Error::custom),
            "AntennaResourceID_Type" => serde_json::from_value(value)
                .map(Self::AntennaResourceIdType)
                .map_err(serde::de::Error::custom),
            "ApprovalAuthorityID_Type" => serde_json::from_value(value)
                .map(Self::ApprovalAuthorityIdType)
                .map_err(serde::de::Error::custom),
            "ApprovalPolicyID_Type" => serde_json::from_value(value)
                .map(Self::ApprovalPolicyIdType)
                .map_err(serde::de::Error::custom),
            "AssessmentID_Type" => serde_json::from_value(value)
                .map(Self::AssessmentIdType)
                .map_err(serde::de::Error::custom),
            "AuthorizationID_Type" => serde_json::from_value(value)
                .map(Self::AuthorizationIdType)
                .map_err(serde::de::Error::custom),
            "AutonomousPlanningActionID_Type" => serde_json::from_value(value)
                .map(Self::AutonomousPlanningActionIdType)
                .map_err(serde::de::Error::custom),
            "BIT_ID_Type" => serde_json::from_value(value)
                .map(Self::BitIdType)
                .map_err(serde::de::Error::custom),
            "BeamID_Type" => serde_json::from_value(value)
                .map(Self::BeamIdType)
                .map_err(serde::de::Error::custom),
            "CalibrationID_Type" => serde_json::from_value(value)
                .map(Self::CalibrationIdType)
                .map_err(serde::de::Error::custom),
            "CapabilityConfigurationID_Type" => serde_json::from_value(value)
                .map(Self::CapabilityConfigurationIdType)
                .map_err(serde::de::Error::custom),
            "CapabilityCoverageAreaID_Type" => serde_json::from_value(value)
                .map(Self::CapabilityCoverageAreaIdType)
                .map_err(serde::de::Error::custom),
            "CapabilityCoverageAreaProcessID_Type" => serde_json::from_value(value)
                .map(Self::CapabilityCoverageAreaProcessIdType)
                .map_err(serde::de::Error::custom),
            "CapabilityID_Type" => serde_json::from_value(value)
                .map(Self::CapabilityIdType)
                .map_err(serde::de::Error::custom),
            "CargoID_Type" => serde_json::from_value(value)
                .map(Self::CargoIdType)
                .map_err(serde::de::Error::custom),
            "CommConfigurationID_Type" => serde_json::from_value(value)
                .map(Self::CommConfigurationIdType)
                .map_err(serde::de::Error::custom),
            "CommEffectivityID_Type" => serde_json::from_value(value)
                .map(Self::CommEffectivityIdType)
                .map_err(serde::de::Error::custom),
            "CommPointingID_Type" => serde_json::from_value(value)
                .map(Self::CommPointingIdType)
                .map_err(serde::de::Error::custom),
            "CommProfileID_Type" => serde_json::from_value(value)
                .map(Self::CommProfileIdType)
                .map_err(serde::de::Error::custom),
            "CommSupportID_Type" => serde_json::from_value(value)
                .map(Self::CommSupportIdType)
                .map_err(serde::de::Error::custom),
            "CommSupportPlanCommandID_Type" => serde_json::from_value(value)
                .map(Self::CommSupportPlanCommandIdType)
                .map_err(serde::de::Error::custom),
            "CommSupportPlanID_Type" => serde_json::from_value(value)
                .map(Self::CommSupportPlanIdType)
                .map_err(serde::de::Error::custom),
            "CommSupportPlanValidationID_Type" => serde_json::from_value(value)
                .map(Self::CommSupportPlanValidationIdType)
                .map_err(serde::de::Error::custom),
            "CommTerminalPlanID_Type" => serde_json::from_value(value)
                .map(Self::CommTerminalPlanIdType)
                .map_err(serde::de::Error::custom),
            "CommUserIO_ID_Type" => serde_json::from_value(value)
                .map(Self::CommUserIoIdType)
                .map_err(serde::de::Error::custom),
            "CommUserLinkID_Type" => serde_json::from_value(value)
                .map(Self::CommUserLinkIdType)
                .map_err(serde::de::Error::custom),
            "CommWindowID_Type" => serde_json::from_value(value)
                .map(Self::CommWindowIdType)
                .map_err(serde::de::Error::custom),
            "CommandID_Type" => serde_json::from_value(value)
                .map(Self::CommandIdType)
                .map_err(serde::de::Error::custom),
            "ComponentConfigurationID_Type" => serde_json::from_value(value)
                .map(Self::ComponentConfigurationIdType)
                .map_err(serde::de::Error::custom),
            "ComponentElementID_Type" => serde_json::from_value(value)
                .map(Self::ComponentElementIdType)
                .map_err(serde::de::Error::custom),
            "ComponentID_Type" => serde_json::from_value(value)
                .map(Self::ComponentIdType)
                .map_err(serde::de::Error::custom),
            "ConflictID_Type" => serde_json::from_value(value)
                .map(Self::ConflictIdType)
                .map_err(serde::de::Error::custom),
            "CoordinatedRequirementSetID_Type" => serde_json::from_value(value)
                .map(Self::CoordinatedRequirementSetIdType)
                .map_err(serde::de::Error::custom),
            "CorrelatedEntityID_Type" => serde_json::from_value(value)
                .map(Self::CorrelatedEntityIdType)
                .map_err(serde::de::Error::custom),
            "CounterSpaceDeconflictionID_Type" => serde_json::from_value(value)
                .map(Self::CounterSpaceDeconflictionIdType)
                .map_err(serde::de::Error::custom),
            "CounterSpaceReportID_Type" => serde_json::from_value(value)
                .map(Self::CounterSpaceReportIdType)
                .map_err(serde::de::Error::custom),
            "CryptoPortID_Type" => serde_json::from_value(value)
                .map(Self::CryptoPortIdType)
                .map_err(serde::de::Error::custom),
            "CryptoSlotGroupID_Type" => serde_json::from_value(value)
                .map(Self::CryptoSlotGroupIdType)
                .map_err(serde::de::Error::custom),
            "DLZ_ID_Type" => serde_json::from_value(value)
                .map(Self::DlzIdType)
                .map_err(serde::de::Error::custom),
            "DMPI_DesignationID_Type" => serde_json::from_value(value)
                .map(Self::DmpiDesignationIdType)
                .map_err(serde::de::Error::custom),
            "DMPI_ID_Type" => serde_json::from_value(value)
                .map(Self::DmpiIdType)
                .map_err(serde::de::Error::custom),
            "DMPI_PatternID_Type" => serde_json::from_value(value)
                .map(Self::DmpiPatternIdType)
                .map_err(serde::de::Error::custom),
            "DamageAssessmentID_Type" => serde_json::from_value(value)
                .map(Self::DamageAssessmentIdType)
                .map_err(serde::de::Error::custom),
            "DamageEstimateID_Type" => serde_json::from_value(value)
                .map(Self::DamageEstimateIdType)
                .map_err(serde::de::Error::custom),
            "DamageReportID_Type" => serde_json::from_value(value)
                .map(Self::DamageReportIdType)
                .map_err(serde::de::Error::custom),
            "DamageSummaryID_Type" => serde_json::from_value(value)
                .map(Self::DamageSummaryIdType)
                .map_err(serde::de::Error::custom),
            "DataDestinationID_Type" => serde_json::from_value(value)
                .map(Self::DataDestinationIdType)
                .map_err(serde::de::Error::custom),
            "DataPlanID_Type" => serde_json::from_value(value)
                .map(Self::DataPlanIdType)
                .map_err(serde::de::Error::custom),
            "DataRecordInstanceID_Type" => serde_json::from_value(value)
                .map(Self::DataRecordInstanceIdType)
                .map_err(serde::de::Error::custom),
            "DataRecordListID_Type" => serde_json::from_value(value)
                .map(Self::DataRecordListIdType)
                .map_err(serde::de::Error::custom),
            "DesignationID_Type" => serde_json::from_value(value)
                .map(Self::DesignationIdType)
                .map_err(serde::de::Error::custom),
            "DwellID_Type" => serde_json::from_value(value)
                .map(Self::DwellIdType)
                .map_err(serde::de::Error::custom),
            "EEI_ID_Type" => serde_json::from_value(value)
                .map(Self::EeiIdType)
                .map_err(serde::de::Error::custom),
            "EW_ConfigurationID_Type" => serde_json::from_value(value)
                .map(Self::EwConfigurationIdType)
                .map_err(serde::de::Error::custom),
            "EarthOrientationParametersID_Type" => serde_json::from_value(value)
                .map(Self::EarthOrientationParametersIdType)
                .map_err(serde::de::Error::custom),
            "EffectID_Type" => serde_json::from_value(value)
                .map(Self::EffectIdType)
                .map_err(serde::de::Error::custom),
            "EffectPlanCommandID_Type" => serde_json::from_value(value)
                .map(Self::EffectPlanCommandIdType)
                .map_err(serde::de::Error::custom),
            "EffectPlanID_Type" => serde_json::from_value(value)
                .map(Self::EffectPlanIdType)
                .map_err(serde::de::Error::custom),
            "EffectPlanValidationID_Type" => serde_json::from_value(value)
                .map(Self::EffectPlanValidationIdType)
                .map_err(serde::de::Error::custom),
            "EmitterModeID_Type" => serde_json::from_value(value)
                .map(Self::EmitterModeIdType)
                .map_err(serde::de::Error::custom),
            "EmitterRecordID_Type" => serde_json::from_value(value)
                .map(Self::EmitterRecordIdType)
                .map_err(serde::de::Error::custom),
            "EntityID_Type" => serde_json::from_value(value)
                .map(Self::EntityIdType)
                .map_err(serde::de::Error::custom),
            "EntityOrbitalCSO_ID_Type" => serde_json::from_value(value)
                .map(Self::EntityOrbitalCsoIdType)
                .map_err(serde::de::Error::custom),
            "EntityOrbitalElementSetID_Type" => serde_json::from_value(value)
                .map(Self::EntityOrbitalElementSetIdType)
                .map_err(serde::de::Error::custom),
            "EntityOrbitalEphemerisID_Type" => serde_json::from_value(value)
                .map(Self::EntityOrbitalEphemerisIdType)
                .map_err(serde::de::Error::custom),
            "EntityOrbitalVCM_ID_Type" => serde_json::from_value(value)
                .map(Self::EntityOrbitalVcmIdType)
                .map_err(serde::de::Error::custom),
            "ExecutionPlanSetID_Type" => serde_json::from_value(value)
                .map(Self::ExecutionPlanSetIdType)
                .map_err(serde::de::Error::custom),
            "ExternalRequestedDataID_Type" => serde_json::from_value(value)
                .map(Self::ExternalRequestedDataIdType)
                .map_err(serde::de::Error::custom),
            "FacilityCharacteristicsID_Type" => serde_json::from_value(value)
                .map(Self::FacilityCharacteristicsIdType)
                .map_err(serde::de::Error::custom),
            "FacilityRecordID_Type" => serde_json::from_value(value)
                .map(Self::FacilityRecordIdType)
                .map_err(serde::de::Error::custom),
            "FaultID_Type" => serde_json::from_value(value)
                .map(Self::FaultIdType)
                .map_err(serde::de::Error::custom),
            "FileLocationID_Type" => serde_json::from_value(value)
                .map(Self::FileLocationIdType)
                .map_err(serde::de::Error::custom),
            "FileMetadataID_Type" => serde_json::from_value(value)
                .map(Self::FileMetadataIdType)
                .map_err(serde::de::Error::custom),
            "FilterID_Type" => serde_json::from_value(value)
                .map(Self::FilterIdType)
                .map_err(serde::de::Error::custom),
            "FlightRuleID_Type" => serde_json::from_value(value)
                .map(Self::FlightRuleIdType)
                .map_err(serde::de::Error::custom),
            "ForeignKeyMapID_Type" => serde_json::from_value(value)
                .map(Self::ForeignKeyMapIdType)
                .map_err(serde::de::Error::custom),
            "ForeignKeyPairID_Type" => serde_json::from_value(value)
                .map(Self::ForeignKeyPairIdType)
                .map_err(serde::de::Error::custom),
            "FunctionID_Type" => serde_json::from_value(value)
                .map(Self::FunctionIdType)
                .map_err(serde::de::Error::custom),
            "GimbalAxisID_Type" => serde_json::from_value(value)
                .map(Self::GimbalAxisIdType)
                .map_err(serde::de::Error::custom),
            "IFF_MeasurementAndDataMessageID_Type" => serde_json::from_value(value)
                .map(Self::IffMeasurementAndDataMessageIdType)
                .map_err(serde::de::Error::custom),
            "IFF_MeasurementAndDataReportMessageID_Type" => serde_json::from_value(value)
                .map(Self::IffMeasurementAndDataReportMessageIdType)
                .map_err(serde::de::Error::custom),
            "IFF_MeasurementID_Type" => serde_json::from_value(value)
                .map(Self::IffMeasurementIdType)
                .map_err(serde::de::Error::custom),
            "JamStrobeReportID_Type" => serde_json::from_value(value)
                .map(Self::JamStrobeReportIdType)
                .map_err(serde::de::Error::custom),
            "LAR_ID_Type" => serde_json::from_value(value)
                .map(Self::LarIdType)
                .map_err(serde::de::Error::custom),
            "LandCharacteristicsID_Type" => serde_json::from_value(value)
                .map(Self::LandCharacteristicsIdType)
                .map_err(serde::de::Error::custom),
            "LandRecordID_Type" => serde_json::from_value(value)
                .map(Self::LandRecordIdType)
                .map_err(serde::de::Error::custom),
            "LaunchObjectID_Type" => serde_json::from_value(value)
                .map(Self::LaunchObjectIdType)
                .map_err(serde::de::Error::custom),
            "LaunchObservationID_Type" => serde_json::from_value(value)
                .map(Self::LaunchObservationIdType)
                .map_err(serde::de::Error::custom),
            "MDF_ID_Type" => serde_json::from_value(value)
                .map(Self::MdfIdType)
                .map_err(serde::de::Error::custom),
            "MDF_SubCategoryFileID_Type" => serde_json::from_value(value)
                .map(Self::MdfSubCategoryFileIdType)
                .map_err(serde::de::Error::custom),
            "MDF_SubCategoryID_Type" => serde_json::from_value(value)
                .map(Self::MdfSubCategoryIdType)
                .map_err(serde::de::Error::custom),
            "MeasurementGroupID_Type" => serde_json::from_value(value)
                .map(Self::MeasurementGroupIdType)
                .map_err(serde::de::Error::custom),
            "MeasurementID_Type" => serde_json::from_value(value)
                .map(Self::MeasurementIdType)
                .map_err(serde::de::Error::custom),
            "MessageConfigurationID_Type" => serde_json::from_value(value)
                .map(Self::MessageConfigurationIdType)
                .map_err(serde::de::Error::custom),
            "MessageTransmissionFilterRecordID_Type" => serde_json::from_value(value)
                .map(Self::MessageTransmissionFilterRecordIdType)
                .map_err(serde::de::Error::custom),
            "MessageTransmissionListID_Type" => serde_json::from_value(value)
                .map(Self::MessageTransmissionListIdType)
                .map_err(serde::de::Error::custom),
            "MetadataID_Type" => serde_json::from_value(value)
                .map(Self::MetadataIdType)
                .map_err(serde::de::Error::custom),
            "MissileCharacteristicsID_Type" => serde_json::from_value(value)
                .map(Self::MissileCharacteristicsIdType)
                .map_err(serde::de::Error::custom),
            "MissileRecordID_Type" => serde_json::from_value(value)
                .map(Self::MissileRecordIdType)
                .map_err(serde::de::Error::custom),
            "MissionContingencyAlertID_Type" => serde_json::from_value(value)
                .map(Self::MissionContingencyAlertIdType)
                .map_err(serde::de::Error::custom),
            "MissionEnvironmentOverrideID_Type" => serde_json::from_value(value)
                .map(Self::MissionEnvironmentOverrideIdType)
                .map_err(serde::de::Error::custom),
            "MissionID_Type" => serde_json::from_value(value)
                .map(Self::MissionIdType)
                .map_err(serde::de::Error::custom),
            "MissionPlanActivationCommandID_Type" => serde_json::from_value(value)
                .map(Self::MissionPlanActivationCommandIdType)
                .map_err(serde::de::Error::custom),
            "MissionPlanCommandID_Type" => serde_json::from_value(value)
                .map(Self::MissionPlanCommandIdType)
                .map_err(serde::de::Error::custom),
            "MissionPlanID_Type" => serde_json::from_value(value)
                .map(Self::MissionPlanIdType)
                .map_err(serde::de::Error::custom),
            "MissionPlanMetricsReportID_Type" => serde_json::from_value(value)
                .map(Self::MissionPlanMetricsReportIdType)
                .map_err(serde::de::Error::custom),
            "MissionPlanValidationID_Type" => serde_json::from_value(value)
                .map(Self::MissionPlanValidationIdType)
                .map_err(serde::de::Error::custom),
            "MissionPlanningOrchestrationID_Type" => serde_json::from_value(value)
                .map(Self::MissionPlanningOrchestrationIdType)
                .map_err(serde::de::Error::custom),
            "MissionValidationID_Type" => serde_json::from_value(value)
                .map(Self::MissionValidationIdType)
                .map_err(serde::de::Error::custom),
            "MultistaticEmitterDataID_Type" => serde_json::from_value(value)
                .map(Self::MultistaticEmitterDataIdType)
                .map_err(serde::de::Error::custom),
            "MultistaticEmitterID_Type" => serde_json::from_value(value)
                .map(Self::MultistaticEmitterIdType)
                .map_err(serde::de::Error::custom),
            "MvAnomalyEventID_Type" => serde_json::from_value(value)
                .map(Self::MvAnomalyEventIdType)
                .map_err(serde::de::Error::custom),
            "NetworkLinkID_Type" => serde_json::from_value(value)
                .map(Self::NetworkLinkIdType)
                .map_err(serde::de::Error::custom),
            "NotificationID_Type" => serde_json::from_value(value)
                .map(Self::NotificationIdType)
                .map_err(serde::de::Error::custom),
            "OB_CorrelationRecordID_Type" => serde_json::from_value(value)
                .map(Self::ObCorrelationRecordIdType)
                .map_err(serde::de::Error::custom),
            "ObservationMeasurementID_Type" => serde_json::from_value(value)
                .map(Self::ObservationMeasurementIdType)
                .map_err(serde::de::Error::custom),
            "ObservationMeasurementReportID_Type" => serde_json::from_value(value)
                .map(Self::ObservationMeasurementReportIdType)
                .map_err(serde::de::Error::custom),
            "ObservationReportID_Type" => serde_json::from_value(value)
                .map(Self::ObservationReportIdType)
                .map_err(serde::de::Error::custom),
            "OpLineID_Type" => serde_json::from_value(value)
                .map(Self::OpLineIdType)
                .map_err(serde::de::Error::custom),
            "OpPointID_Type" => serde_json::from_value(value)
                .map(Self::OpPointIdType)
                .map_err(serde::de::Error::custom),
            "OpRoutingID_Type" => serde_json::from_value(value)
                .map(Self::OpRoutingIdType)
                .map_err(serde::de::Error::custom),
            "OpVolumeID_Type" => serde_json::from_value(value)
                .map(Self::OpVolumeIdType)
                .map_err(serde::de::Error::custom),
            "OpZoneID_Type" => serde_json::from_value(value)
                .map(Self::OpZoneIdType)
                .map_err(serde::de::Error::custom),
            "OperatorActionID_Type" => serde_json::from_value(value)
                .map(Self::OperatorActionIdType)
                .map_err(serde::de::Error::custom),
            "OperatorAnnotationID_Type" => serde_json::from_value(value)
                .map(Self::OperatorAnnotationIdType)
                .map_err(serde::de::Error::custom),
            "OperatorID_Type" => serde_json::from_value(value)
                .map(Self::OperatorIdType)
                .map_err(serde::de::Error::custom),
            "OperatorLocationOfInterestID_Type" => serde_json::from_value(value)
                .map(Self::OperatorLocationOfInterestIdType)
                .map_err(serde::de::Error::custom),
            "OperatorNotificationAckID_Type" => serde_json::from_value(value)
                .map(Self::OperatorNotificationAckIdType)
                .map_err(serde::de::Error::custom),
            "OperatorNotificationID_Type" => serde_json::from_value(value)
                .map(Self::OperatorNotificationIdType)
                .map_err(serde::de::Error::custom),
            "OperatorNotificationOverrideID_Type" => serde_json::from_value(value)
                .map(Self::OperatorNotificationOverrideIdType)
                .map_err(serde::de::Error::custom),
            "OperatorRoleID_Type" => serde_json::from_value(value)
                .map(Self::OperatorRoleIdType)
                .map_err(serde::de::Error::custom),
            "OrbitActivityPlanCommandID_Type" => serde_json::from_value(value)
                .map(Self::OrbitActivityPlanCommandIdType)
                .map_err(serde::de::Error::custom),
            "OrbitActivityPlanID_Type" => serde_json::from_value(value)
                .map(Self::OrbitActivityPlanIdType)
                .map_err(serde::de::Error::custom),
            "OrbitActivityPlanValidationID_Type" => serde_json::from_value(value)
                .map(Self::OrbitActivityPlanValidationIdType)
                .map_err(serde::de::Error::custom),
            "OrbitConjunctionAssessmentID_Type" => serde_json::from_value(value)
                .map(Self::OrbitConjunctionAssessmentIdType)
                .map_err(serde::de::Error::custom),
            "OrbitKinematicsSequenceID_Type" => serde_json::from_value(value)
                .map(Self::OrbitKinematicsSequenceIdType)
                .map_err(serde::de::Error::custom),
            "OrbitManeuverSegmentID_Type" => serde_json::from_value(value)
                .map(Self::OrbitManeuverSegmentIdType)
                .map_err(serde::de::Error::custom),
            "OrbitMetricsID_Type" => serde_json::from_value(value)
                .map(Self::OrbitMetricsIdType)
                .map_err(serde::de::Error::custom),
            "OrbitPlanCommandID_Type" => serde_json::from_value(value)
                .map(Self::OrbitPlanCommandIdType)
                .map_err(serde::de::Error::custom),
            "OrbitPlanID_Type" => serde_json::from_value(value)
                .map(Self::OrbitPlanIdType)
                .map_err(serde::de::Error::custom),
            "OrbitPlanValidationID_Type" => serde_json::from_value(value)
                .map(Self::OrbitPlanValidationIdType)
                .map_err(serde::de::Error::custom),
            "OrbitalConjunctionID_Type" => serde_json::from_value(value)
                .map(Self::OrbitalConjunctionIdType)
                .map_err(serde::de::Error::custom),
            "OrbitalDebrisCloudID_Type" => serde_json::from_value(value)
                .map(Self::OrbitalDebrisCloudIdType)
                .map_err(serde::de::Error::custom),
            "OrbitalDebrisEstimateID_Type" => serde_json::from_value(value)
                .map(Self::OrbitalDebrisEstimateIdType)
                .map_err(serde::de::Error::custom),
            "OrbitalDebrisID_Type" => serde_json::from_value(value)
                .map(Self::OrbitalDebrisIdType)
                .map_err(serde::de::Error::custom),
            "OrbitalManeuverID_Type" => serde_json::from_value(value)
                .map(Self::OrbitalManeuverIdType)
                .map_err(serde::de::Error::custom),
            "OrderOfBattleID_Type" => serde_json::from_value(value)
                .map(Self::OrderOfBattleIdType)
                .map_err(serde::de::Error::custom),
            "OrdersMetadataID_Type" => serde_json::from_value(value)
                .map(Self::OrdersMetadataIdType)
                .map_err(serde::de::Error::custom),
            "PO_ApertureID_Type" => serde_json::from_value(value)
                .map(Self::PoApertureIdType)
                .map_err(serde::de::Error::custom),
            "PO_FPA_ChannelID_Type" => serde_json::from_value(value)
                .map(Self::PoFpaChannelIdType)
                .map_err(serde::de::Error::custom),
            "PO_FPA_ID_Type" => serde_json::from_value(value)
                .map(Self::PoFpaIdType)
                .map_err(serde::de::Error::custom),
            "PO_FilterID_Type" => serde_json::from_value(value)
                .map(Self::PoFilterIdType)
                .map_err(serde::de::Error::custom),
            "PO_LensID_Type" => serde_json::from_value(value)
                .map(Self::PoLensIdType)
                .map_err(serde::de::Error::custom),
            "PO_ProcessingStageID_Type" => serde_json::from_value(value)
                .map(Self::PoProcessingStageIdType)
                .map_err(serde::de::Error::custom),
            "PO_ProductGeneratorID_Type" => serde_json::from_value(value)
                .map(Self::PoProductGeneratorIdType)
                .map_err(serde::de::Error::custom),
            "PO_ProductGeneratorOutputID_Type" => serde_json::from_value(value)
                .map(Self::PoProductGeneratorOutputIdType)
                .map_err(serde::de::Error::custom),
            "PO_WindowID_Type" => serde_json::from_value(value)
                .map(Self::PoWindowIdType)
                .map_err(serde::de::Error::custom),
            "PackageID_Type" => serde_json::from_value(value)
                .map(Self::PackageIdType)
                .map_err(serde::de::Error::custom),
            "PackageReadinessID_Type" => serde_json::from_value(value)
                .map(Self::PackageReadinessIdType)
                .map_err(serde::de::Error::custom),
            "ParameterID_Type" => serde_json::from_value(value)
                .map(Self::ParameterIdType)
                .map_err(serde::de::Error::custom),
            "PathID_Type" => serde_json::from_value(value)
                .map(Self::PathIdType)
                .map_err(serde::de::Error::custom),
            "PlanScoresID_Type" => serde_json::from_value(value)
                .map(Self::PlanScoresIdType)
                .map_err(serde::de::Error::custom),
            "PlannedActivityEventID_Type" => serde_json::from_value(value)
                .map(Self::PlannedActivityEventIdType)
                .map_err(serde::de::Error::custom),
            "PlannedActivityID_Type" => serde_json::from_value(value)
                .map(Self::PlannedActivityIdType)
                .map_err(serde::de::Error::custom),
            "PlanningFunctionID_Type" => serde_json::from_value(value)
                .map(Self::PlanningFunctionIdType)
                .map_err(serde::de::Error::custom),
            "PlanningProcessID_Type" => serde_json::from_value(value)
                .map(Self::PlanningProcessIdType)
                .map_err(serde::de::Error::custom),
            "PlatformRecordID_Type" => serde_json::from_value(value)
                .map(Self::PlatformRecordIdType)
                .map_err(serde::de::Error::custom),
            "PlatformReportID_Type" => serde_json::from_value(value)
                .map(Self::PlatformReportIdType)
                .map_err(serde::de::Error::custom),
            "PrioritizationListID_Type" => serde_json::from_value(value)
                .map(Self::PrioritizationListIdType)
                .map_err(serde::de::Error::custom),
            "ProductDownloadPlanID_Type" => serde_json::from_value(value)
                .map(Self::ProductDownloadPlanIdType)
                .map_err(serde::de::Error::custom),
            "ProductDownloadReportID_Type" => serde_json::from_value(value)
                .map(Self::ProductDownloadReportIdType)
                .map_err(serde::de::Error::custom),
            "ProductLocationID_Type" => serde_json::from_value(value)
                .map(Self::ProductLocationIdType)
                .map_err(serde::de::Error::custom),
            "ProductMetadataID_Type" => serde_json::from_value(value)
                .map(Self::ProductMetadataIdType)
                .map_err(serde::de::Error::custom),
            "ProductOrFileClassificationPlanID_Type" => serde_json::from_value(value)
                .map(Self::ProductOrFileClassificationPlanIdType)
                .map_err(serde::de::Error::custom),
            "ProductOrFileClassificationReportID_Type" => serde_json::from_value(value)
                .map(Self::ProductOrFileClassificationReportIdType)
                .map_err(serde::de::Error::custom),
            "ProductOrFileDisseminationConfigurationID_Type" => serde_json::from_value(value)
                .map(Self::ProductOrFileDisseminationConfigurationIdType)
                .map_err(serde::de::Error::custom),
            "ProductOrFileDisseminationDestinationID_Type" => serde_json::from_value(value)
                .map(Self::ProductOrFileDisseminationDestinationIdType)
                .map_err(serde::de::Error::custom),
            "ProductOrFileDisseminationPlanID_Type" => serde_json::from_value(value)
                .map(Self::ProductOrFileDisseminationPlanIdType)
                .map_err(serde::de::Error::custom),
            "ProductOrFileDisseminationReportID_Type" => serde_json::from_value(value)
                .map(Self::ProductOrFileDisseminationReportIdType)
                .map_err(serde::de::Error::custom),
            "ProductProcessingPlanID_Type" => serde_json::from_value(value)
                .map(Self::ProductProcessingPlanIdType)
                .map_err(serde::de::Error::custom),
            "ProductProcessingReportID_Type" => serde_json::from_value(value)
                .map(Self::ProductProcessingReportIdType)
                .map_err(serde::de::Error::custom),
            "PropagatorSettingsID_Type" => serde_json::from_value(value)
                .map(Self::PropagatorSettingsIdType)
                .map_err(serde::de::Error::custom),
            "PulseDataID_Type" => serde_json::from_value(value)
                .map(Self::PulseDataIdType)
                .map_err(serde::de::Error::custom),
            "RDMA_InitializeID_Type" => serde_json::from_value(value)
                .map(Self::RdmaInitializeIdType)
                .map_err(serde::de::Error::custom),
            "RDMA_InitializeSetupID_Type" => serde_json::from_value(value)
                .map(Self::RdmaInitializeSetupIdType)
                .map_err(serde::de::Error::custom),
            "RF_ProfileID_Type" => serde_json::from_value(value)
                .map(Self::RfProfileIdType)
                .map_err(serde::de::Error::custom),
            "RF_ReportID_Type" => serde_json::from_value(value)
                .map(Self::RfReportIdType)
                .map_err(serde::de::Error::custom),
            "RF_ResourceAllocationID_Type" => serde_json::from_value(value)
                .map(Self::RfResourceAllocationIdType)
                .map_err(serde::de::Error::custom),
            "RadarAltimeterReportID_Type" => serde_json::from_value(value)
                .map(Self::RadarAltimeterReportIdType)
                .map_err(serde::de::Error::custom),
            "RecordID_Type" => serde_json::from_value(value)
                .map(Self::RecordIdType)
                .map_err(serde::de::Error::custom),
            "ReferenceFrameID_Type" => serde_json::from_value(value)
                .map(Self::ReferenceFrameIdType)
                .map_err(serde::de::Error::custom),
            "RelationshipDesignationID_Type" => serde_json::from_value(value)
                .map(Self::RelationshipDesignationIdType)
                .map_err(serde::de::Error::custom),
            "RequestID_Type" => serde_json::from_value(value)
                .map(Self::RequestIdType)
                .map_err(serde::de::Error::custom),
            "RequirementAllocationWindowID_Type" => serde_json::from_value(value)
                .map(Self::RequirementAllocationWindowIdType)
                .map_err(serde::de::Error::custom),
            "RequirementMetricsID_Type" => serde_json::from_value(value)
                .map(Self::RequirementMetricsIdType)
                .map_err(serde::de::Error::custom),
            "RequirementOptionsID_Type" => serde_json::from_value(value)
                .map(Self::RequirementOptionsIdType)
                .map_err(serde::de::Error::custom),
            "RequirementSetID_Type" => serde_json::from_value(value)
                .map(Self::RequirementSetIdType)
                .map_err(serde::de::Error::custom),
            "RequirementsTemplateID_Type" => serde_json::from_value(value)
                .map(Self::RequirementsTemplateIdType)
                .map_err(serde::de::Error::custom),
            "ResponseID_Type" => serde_json::from_value(value)
                .map(Self::ResponseIdType)
                .map_err(serde::de::Error::custom),
            "ResponsePlanCommandID_Type" => serde_json::from_value(value)
                .map(Self::ResponsePlanCommandIdType)
                .map_err(serde::de::Error::custom),
            "ResponsePlanID_Type" => serde_json::from_value(value)
                .map(Self::ResponsePlanIdType)
                .map_err(serde::de::Error::custom),
            "ResponsePlanValidationID_Type" => serde_json::from_value(value)
                .map(Self::ResponsePlanValidationIdType)
                .map_err(serde::de::Error::custom),
            "RouteActivityPlanCommandID_Type" => serde_json::from_value(value)
                .map(Self::RouteActivityPlanCommandIdType)
                .map_err(serde::de::Error::custom),
            "RouteActivityPlanID_Type" => serde_json::from_value(value)
                .map(Self::RouteActivityPlanIdType)
                .map_err(serde::de::Error::custom),
            "RouteActivityPlanValidationID_Type" => serde_json::from_value(value)
                .map(Self::RouteActivityPlanValidationIdType)
                .map_err(serde::de::Error::custom),
            "RouteMetricsID_Type" => serde_json::from_value(value)
                .map(Self::RouteMetricsIdType)
                .map_err(serde::de::Error::custom),
            "RoutePlanCommandID_Type" => serde_json::from_value(value)
                .map(Self::RoutePlanCommandIdType)
                .map_err(serde::de::Error::custom),
            "RoutePlanID_Type" => serde_json::from_value(value)
                .map(Self::RoutePlanIdType)
                .map_err(serde::de::Error::custom),
            "RoutePlanValidationID_Type" => serde_json::from_value(value)
                .map(Self::RoutePlanValidationIdType)
                .map_err(serde::de::Error::custom),
            "RunwayID_Type" => serde_json::from_value(value)
                .map(Self::RunwayIdType)
                .map_err(serde::de::Error::custom),
            "SatellitePhysicalConfigurationID_Type" => serde_json::from_value(value)
                .map(Self::SatellitePhysicalConfigurationIdType)
                .map_err(serde::de::Error::custom),
            "ScoringProcessID_Type" => serde_json::from_value(value)
                .map(Self::ScoringProcessIdType)
                .map_err(serde::de::Error::custom),
            "SeaSubsurfaceCharacteristicsID_Type" => serde_json::from_value(value)
                .map(Self::SeaSubsurfaceCharacteristicsIdType)
                .map_err(serde::de::Error::custom),
            "SeaSubsurfaceRecordID_Type" => serde_json::from_value(value)
                .map(Self::SeaSubsurfaceRecordIdType)
                .map_err(serde::de::Error::custom),
            "SeaSurfaceCharacteristicsID_Type" => serde_json::from_value(value)
                .map(Self::SeaSurfaceCharacteristicsIdType)
                .map_err(serde::de::Error::custom),
            "SeaSurfaceRecordID_Type" => serde_json::from_value(value)
                .map(Self::SeaSurfaceRecordIdType)
                .map_err(serde::de::Error::custom),
            "SegmentID_Type" => serde_json::from_value(value)
                .map(Self::SegmentIdType)
                .map_err(serde::de::Error::custom),
            "SensorID_Type" => serde_json::from_value(value)
                .map(Self::SensorIdType)
                .map_err(serde::de::Error::custom),
            "ServiceErrorReportID_Type" => serde_json::from_value(value)
                .map(Self::ServiceErrorReportIdType)
                .map_err(serde::de::Error::custom),
            "ServiceID_Type" => serde_json::from_value(value)
                .map(Self::ServiceIdType)
                .map_err(serde::de::Error::custom),
            "SignalID_Type" => serde_json::from_value(value)
                .map(Self::SignalIdType)
                .map_err(serde::de::Error::custom),
            "SignalReportID_Type" => serde_json::from_value(value)
                .map(Self::SignalReportIdType)
                .map_err(serde::de::Error::custom),
            "SpaceCharacteristicsID_Type" => serde_json::from_value(value)
                .map(Self::SpaceCharacteristicsIdType)
                .map_err(serde::de::Error::custom),
            "SpaceRecordID_Type" => serde_json::from_value(value)
                .map(Self::SpaceRecordIdType)
                .map_err(serde::de::Error::custom),
            "SpaceWeatherID_Type" => serde_json::from_value(value)
                .map(Self::SpaceWeatherIdType)
                .map_err(serde::de::Error::custom),
            "SpectralDensityReportID_Type" => serde_json::from_value(value)
                .map(Self::SpectralDensityReportIdType)
                .map_err(serde::de::Error::custom),
            "StoreLoadoutConfigurationID_Type" => serde_json::from_value(value)
                .map(Self::StoreLoadoutConfigurationIdType)
                .map_err(serde::de::Error::custom),
            "StoreManagementReportID_Type" => serde_json::from_value(value)
                .map(Self::StoreManagementReportIdType)
                .map_err(serde::de::Error::custom),
            "SubCapabilityID_Type" => serde_json::from_value(value)
                .map(Self::SubCapabilityIdType)
                .map_err(serde::de::Error::custom),
            "SubsystemBIT_ConfigurationID_Type" => serde_json::from_value(value)
                .map(Self::SubsystemBitConfigurationIdType)
                .map_err(serde::de::Error::custom),
            "SubsystemCalibrationConfigurationID_Type" => serde_json::from_value(value)
                .map(Self::SubsystemCalibrationConfigurationIdType)
                .map_err(serde::de::Error::custom),
            "SubsystemConfigurationID_Type" => serde_json::from_value(value)
                .map(Self::SubsystemConfigurationIdType)
                .map_err(serde::de::Error::custom),
            "SubsystemID_Type" => serde_json::from_value(value)
                .map(Self::SubsystemIdType)
                .map_err(serde::de::Error::custom),
            "SubsystemMaintenanceConfigurationID_Type" => serde_json::from_value(value)
                .map(Self::SubsystemMaintenanceConfigurationIdType)
                .map_err(serde::de::Error::custom),
            "SubsystemMaintenanceID_Type" => serde_json::from_value(value)
                .map(Self::SubsystemMaintenanceIdType)
                .map_err(serde::de::Error::custom),
            "SupportCapabilityID_Type" => serde_json::from_value(value)
                .map(Self::SupportCapabilityIdType)
                .map_err(serde::de::Error::custom),
            "SupportPlanID_Type" => serde_json::from_value(value)
                .map(Self::SupportPlanIdType)
                .map_err(serde::de::Error::custom),
            "SupportedResolutionID_Type" => serde_json::from_value(value)
                .map(Self::SupportedResolutionIdType)
                .map_err(serde::de::Error::custom),
            "SurvivabilityRiskLevelID_Type" => serde_json::from_value(value)
                .map(Self::SurvivabilityRiskLevelIdType)
                .map_err(serde::de::Error::custom),
            "SystemID_Type" => serde_json::from_value(value)
                .map(Self::SystemIdType)
                .map_err(serde::de::Error::custom),
            "SystemOrbitalElementSetID_Type" => serde_json::from_value(value)
                .map(Self::SystemOrbitalElementSetIdType)
                .map_err(serde::de::Error::custom),
            "SystemOrbitalEphemerisID_Type" => serde_json::from_value(value)
                .map(Self::SystemOrbitalEphemerisIdType)
                .map_err(serde::de::Error::custom),
            "SystemOrbitalVCM_ID_Type" => serde_json::from_value(value)
                .map(Self::SystemOrbitalVcmIdType)
                .map_err(serde::de::Error::custom),
            "SystemReadinessID_Type" => serde_json::from_value(value)
                .map(Self::SystemReadinessIdType)
                .map_err(serde::de::Error::custom),
            "TagAssociationID_Type" => serde_json::from_value(value)
                .map(Self::TagAssociationIdType)
                .map_err(serde::de::Error::custom),
            "TaskID_Type" => serde_json::from_value(value)
                .map(Self::TaskIdType)
                .map_err(serde::de::Error::custom),
            "TaskPlanCommandID_Type" => serde_json::from_value(value)
                .map(Self::TaskPlanCommandIdType)
                .map_err(serde::de::Error::custom),
            "TaskPlanID_Type" => serde_json::from_value(value)
                .map(Self::TaskPlanIdType)
                .map_err(serde::de::Error::custom),
            "TaskPlanValidationID_Type" => serde_json::from_value(value)
                .map(Self::TaskPlanValidationIdType)
                .map_err(serde::de::Error::custom),
            "TrackOptionsID_Type" => serde_json::from_value(value)
                .map(Self::TrackOptionsIdType)
                .map_err(serde::de::Error::custom),
            "TurretReportID_Type" => serde_json::from_value(value)
                .map(Self::TurretReportIdType)
                .map_err(serde::de::Error::custom),
            "UMID_AudioID_Type" => serde_json::from_value(value)
                .map(Self::UmidAudioIdType)
                .map_err(serde::de::Error::custom),
            "UMID_DataID_Type" => serde_json::from_value(value)
                .map(Self::UmidDataIdType)
                .map_err(serde::de::Error::custom),
            "UMID_SystemID_Type" => serde_json::from_value(value)
                .map(Self::UmidSystemIdType)
                .map_err(serde::de::Error::custom),
            "UMID_VideoID_Type" => serde_json::from_value(value)
                .map(Self::UmidVideoIdType)
                .map_err(serde::de::Error::custom),
            "UnitID_Type" => serde_json::from_value(value)
                .map(Self::UnitIdType)
                .map_err(serde::de::Error::custom),
            "UnitRecordID_Type" => serde_json::from_value(value)
                .map(Self::UnitRecordIdType)
                .map_err(serde::de::Error::custom),
            "VersionedID_Type" => serde_json::from_value(value)
                .map(Self::VersionedIdType)
                .map_err(serde::de::Error::custom),
            "WeatherDatasetID_Type" => serde_json::from_value(value)
                .map(Self::WeatherDatasetIdType)
                .map_err(serde::de::Error::custom),
            _ => Err(serde::de::Error::custom(format!(
                "illegal derived type {tag}"
            ))),
        }
    }
}

/// Publishable OMS JSON `ServiceLifecycleCommand` global root.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceLifecycleCommand {
    /// Complete message body, including security and header.
    #[serde(rename = "ServiceLifecycleCommand")]
    pub message: ServiceLifecycleCommandMt,
}
impl crate::sealed::Sealed for ServiceLifecycleCommand {}
impl crate::Message for ServiceLifecycleCommand {
    const ROOT: &'static str = "ServiceLifecycleCommand";
    const TOPIC: &'static str = "ServiceLifecycleCommand";
    const PROFILE: &'static str = "002.5.0";
}

/// Publishable OMS JSON `ServiceLifecycleCommandStatus` global root.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceLifecycleCommandStatus {
    /// Complete message body, including security and header.
    #[serde(rename = "ServiceLifecycleCommandStatus")]
    pub message: ServiceLifecycleCommandStatusMt,
}
impl crate::sealed::Sealed for ServiceLifecycleCommandStatus {}
impl crate::Message for ServiceLifecycleCommandStatus {
    const ROOT: &'static str = "ServiceLifecycleCommandStatus";
    const TOPIC: &'static str = "ServiceLifecycleCommandStatus";
    const PROFILE: &'static str = "002.5.0";
}

/// Publishable OMS JSON `ServiceStatus` global root.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceStatus {
    /// Complete message body, including security and header.
    #[serde(rename = "ServiceStatus")]
    pub message: ServiceStatusMt,
}
impl crate::sealed::Sealed for ServiceStatus {}
impl crate::Message for ServiceStatus {
    const ROOT: &'static str = "ServiceStatus";
    const TOPIC: &'static str = "ServiceStatus";
    const PROFILE: &'static str = "002.5.0";
}
