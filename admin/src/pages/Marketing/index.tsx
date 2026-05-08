import { Navigate, Route, Routes, useLocation, useNavigate } from 'react-router-dom'
import { Tabs } from 'antd'
import LeadList from './LeadList'
import CampaignList from './CampaignList'
import CampaignDetail from './CampaignDetail'

const Marketing = () => {
  const navigate = useNavigate()
  const { pathname } = useLocation()
  const activeKey = pathname.includes('/marketing/leads') ? 'leads' : 'campaigns'

  return (
    <div>
      {!pathname.match(/\/marketing\/\d+$/) && (
        <Tabs
          activeKey={activeKey}
          onChange={(key) => navigate(key === 'leads' ? '/marketing/leads' : '/marketing/campaigns')}
          items={[
            { key: 'campaigns', label: '营销活动' },
            { key: 'leads', label: '线索库' },
          ]}
        />
      )}
      <Routes>
        <Route index element={<Navigate to="/marketing/campaigns" replace />} />
        <Route path="campaigns" element={<CampaignList />} />
        <Route path="leads" element={<LeadList />} />
        <Route path=":id" element={<CampaignDetail />} />
      </Routes>
    </div>
  )
}

export default Marketing
