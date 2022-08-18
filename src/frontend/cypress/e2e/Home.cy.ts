import getSymbolFromCurrency from 'currency-symbol-map';
import SessionGateway from '../../gateways/Session.gateway';
import { CypressFields, getElementByField } from '../../utils/Cypress';

describe('Home Page', () => {
  beforeEach(() => {
    cy.visit('/');
  });

  it('should validate the home page', () => {
    getElementByField(CypressFields.HomePage).should('exist');
    getElementByField(CypressFields.ProductCard, getElementByField(CypressFields.ProductList)).should('have.length', 9);

    getElementByField(CypressFields.SessionId).should('contain', SessionGateway.getSession().userId);
  });

  it('should change currency', () => {
    cy.intercept('POST', '/api/currency/convert*').as('convertCurrency');

    getElementByField(CypressFields.CurrencySwitcher).select('EUR');
    getElementByField(CypressFields.ProductCard, getElementByField(CypressFields.ProductList)).should('have.length', 9);

    getElementByField(CypressFields.CurrencySwitcher).should('have.value', 'EUR');

    cy.wait('@convertCurrency');

    getElementByField(CypressFields.ProductCard).should('contain', getSymbolFromCurrency('EUR'));
  });
});
