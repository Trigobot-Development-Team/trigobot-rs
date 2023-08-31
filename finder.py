#!/usr/bin/env python3
import requests
import json
from sys import argv, exit

# MEIC-T 2761663971585
# MEIC-A 2761663971475
degree_id = 2761663971475

if len(argv) < 2:
  print(f"Usage: {argv[0]} <semester: 1|2>")
  exit()

semester = argv[1]

renaming = {
    "GTI": "AID",
    "ASE": "ACPIC",
    "AP-Dei": "AP",
    "BC2": "BC",
    "CSF2": "CSF",
    "CDadosi2": "CD",
    "CCEIC-I": "CCEIC1",
    "DDRS2": "DDRS",
    "PADI": "DAD",
    "QS": "EspSof",
    "DIIC": "EDI",
    "AOSI": "FSI",
    "LN2": "LN",
    "PIV3": "PIV",
    "RGI": "PRI",
    "RSIPR": "RSIHR",
    "PMEIC2": "PMEIC",
    "AOBD": "ADSI",
    "APFSI": "AEmp",
    "AVExe2": "CNV",
    "CMov2": "CMU",
    "CPD2": "CPD",
    "CVI": "CV",
    "SEI": "IEmp",
    "LPro": "LP",
    "TJS": "MDJ",
    "ADI": "PADInt",
    "PF-2": "PF",
    "PSJ": "P3D",
    "OGFI": "SGSI",
    "SDTF2": "SEC",
    "SRMan": "SRM",
    "TIS": "TIDB",
    "DMEIC2": "DMEIC",
}

ignored = ["AExt-I-4", "AExt-II-4", "AExt23", "AExt22"]

response = requests.get(f"https://fenix.tecnico.ulisboa.pt/api/fenix/v1/degrees/{degree_id}/courses?academicTerm=2022/2023").json()

courses = {}

def rename(acronym):
    return renaming.get(acronym, acronym)

def rss_url(acronym):
    return f"https://fenix.tecnico.ulisboa.pt/disciplinas/{acronym}/2022-2023/1-semestre/rss/announcement"

count = 0

for course in response:
    if course["academicTerm"][0] == semester and course["acronym"] not in ignored:
        courses[rename(course["acronym"])] = { "updated": 0, "link": rss_url(course["acronym"])  }
        count += 1

print(f"Added {count} courses")

with open("result.json", "w") as f:
    json.dump(courses, f)
