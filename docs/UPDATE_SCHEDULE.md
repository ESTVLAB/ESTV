# Repository Update Schedule

## Automated Updates (GitHub Actions)

| Schedule | Action | File Updated |
|----------|--------|--------------|
| Every Monday | Stats update | `docs/STATS.md` |
| Weekly | Dependency check | Dependabot PRs |
| On push | CI/CD checks | Build status |

---

## Manual Update Schedule

### Daily (권장)
- [ ] NEWS.md - 새로운 뉴스/발표 추가
- [ ] 작은 문서 개선

### Weekly (필수)
- [ ] DEVELOPMENT.md - 개발 진행상황 업데이트
- [ ] ROADMAP.md - 마일스톤 체크
- [ ] 최소 2-3개 커밋

### Monthly (필수)
- [ ] TOKEN_METRICS.md - 순환 공급량 업데이트
- [ ] PARTNERSHIPS.md - 새 파트너십 추가
- [ ] TEAM.md - 팀 변경사항
- [ ] CHANGELOG.md - 버전 업데이트

### Quarterly
- [ ] Whitepaper 업데이트 (필요시)
- [ ] TOKENOMICS.md 검토
- [ ] 보안 감사 상태 업데이트

---

## Update Commands

### Quick Stats Update
```bash
cd /home/ESTV
./scripts/update-stats.sh
git add docs/STATS.md
git commit -m "chore: update stats"
git push
```

### Full Update
```bash
cd /home/ESTV
# Edit relevant files
git add .
git commit -m "docs: weekly update - [description]"
git push
```

---

## Commit Message Format

```
type: description

Types:
- feat: 새 기능
- fix: 버그 수정
- docs: 문서 업데이트
- chore: 유지보수
- ci: CI/CD 변경
```

---

## What Exchanges Look For

1. **Active Development** - 최근 커밋 활동
2. **Regular Updates** - 주기적인 문서 업데이트
3. **Community Engagement** - 이슈/PR 활동
4. **Version Releases** - 태그된 릴리스

---

## 잔디 유지 팁

- 매일 최소 1개 커밋
- 작은 변경도 개별 커밋
- 의미 있는 커밋 메시지
- 문서 개선, 오타 수정도 커밋

---

*저에게 "업데이트 해줘" 또는 "잔디 심어줘" 라고 하시면 됩니다.*
